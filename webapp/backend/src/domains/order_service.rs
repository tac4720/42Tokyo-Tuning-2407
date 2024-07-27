use chrono::{DateTime, Utc};
use crate::domains::dto::order::OrderWithDetails;
use tokio::try_join;

use super::{
    auth_service::AuthRepository,
    dto::order::{CompletedOrderDto, OrderDto},
    map_service::MapRepository,
    tow_truck_service::TowTruckRepository,
};
use crate::{
    errors::AppError,
    models::order::{CompletedOrder, Order},
};

pub trait OrderRepository {
    async fn find_order_by_id(&self, id: i32) -> Result<Order, AppError>;
    async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError>;
    async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<Order>, AppError>;
    async fn create_order(
        &self,
        customer_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError>;
    async fn update_order_dispatched(
        &self,
        id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
    ) -> Result<(), AppError>;
    async fn create_completed_order(
        &self,
        order_id: i32,
        tow_truck_id: i32,
        completed_time: DateTime<Utc>,
    ) -> Result<(), AppError>;
    async fn get_all_completed_orders(&self) -> Result<Vec<CompletedOrder>, AppError>;
    async fn get_paginated_orders_with_details(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<OrderWithDetails>, AppError>;
}

#[derive(Debug)]
pub struct OrderService<
    T: OrderRepository + std::fmt::Debug,
    U: TowTruckRepository + std::fmt::Debug,
    V: AuthRepository + std::fmt::Debug,
    W: MapRepository + std::fmt::Debug,
> {
    order_repository: T,
    tow_truck_repository: U,
    auth_repository: V,
    map_repository: W,
}

impl<
        T: OrderRepository + std::fmt::Debug,
        U: TowTruckRepository + std::fmt::Debug,
        V: AuthRepository + std::fmt::Debug,
        W: MapRepository + std::fmt::Debug,
    > OrderService<T, U, V, W>
{
    pub fn new(
        order_repository: T,
        tow_truck_repository: U,
        auth_repository: V,
        map_repository: W,
    ) -> Self {
        OrderService {
            order_repository,
            tow_truck_repository,
            auth_repository,
            map_repository,
        }
    }

    pub async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError> {
        self.order_repository
            .update_order_status(order_id, status)
            .await
    }

pub async fn get_order_by_id(&self, id: i32) -> Result<OrderDto, AppError> {
    // Orderを取得
    let order = self.order_repository.find_order_by_id(id).await?;
    
    // まとめてIDを取得
    let client_id = order.client_id;
    let dispatcher_id = order.dispatcher_id;
    let tow_truck_id = order.tow_truck_id;

    // Clientの情報を取得
    let client_future = self.auth_repository.find_user_by_id(client_id);

    // Dispatcherとそのユーザー情報を取得
    let dispatcher_future = async {
        if let Some(dispatcher_id) = dispatcher_id {
            let dispatcher = self.auth_repository.find_dispatcher_by_id(dispatcher_id).await?.unwrap();
            let dispatcher_user = self.auth_repository.find_user_by_id(dispatcher.user_id).await?;
            Ok(Some((dispatcher, dispatcher_user)))
        } else {
            Ok(None)
        }
    };

    // Tow Truckとそのドライバー情報を取得
    let tow_truck_future = async {
        if let Some(tow_truck_id) = tow_truck_id {
            let tow_truck = self.tow_truck_repository.find_tow_truck_by_id(tow_truck_id).await?.unwrap();
            let driver_user = self.auth_repository.find_user_by_id(tow_truck.driver_id).await?;
            Ok(Some((tow_truck, driver_user)))
        } else {
            Ok(None)
        }
    };

    // Area IDを取得
    let area_id_future = self.map_repository.get_area_id_by_node_id(order.node_id);

    // 全てのFutureを並行して実行
    let (client, dispatcher_result, tow_truck_result, area_id) = try_join!(
        client_future,
        dispatcher_future,
        tow_truck_future,
        area_id_future
    )?;

    // 各結果を分解
    let client_username = client.unwrap().username;
    
    let (dispatcher_user_id, dispatcher_username) = if let Some((dispatcher, dispatcher_user)) = dispatcher_result {
        (Some(dispatcher.user_id), Some(dispatcher_user.unwrap().username))
    } else {
        (None, None)
    };

    let (driver_user_id, driver_username) = if let Some((tow_truck, driver_user)) = tow_truck_result {
        (Some(tow_truck.driver_id), Some(driver_user.unwrap().username))
    } else {
        (None, None)
    };

    Ok(OrderDto {
        id: order.id,
        client_id: order.client_id,
        client_username: Some(client_username),
        dispatcher_user_id,
        dispatcher_username,
        driver_user_id,
        driver_username,
        area_id,
        dispatcher_id: order.dispatcher_id,
        tow_truck_id: order.tow_truck_id,
        status: order.status,
        node_id: order.node_id,
        car_value: order.car_value,
        order_time: order.order_time,
        completed_time: order.completed_time,
    })
}

// service
pub async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<OrderDto>, AppError> {
        let orders_with_details = self
            .order_repository
            .get_paginated_orders_with_details(page, page_size, sort_by, sort_order, status, area)
            .await?;

        let results: Vec<OrderDto> = orders_with_details.into_iter().map(|order| {
            OrderDto {
                id: order.id,
                client_id: order.client_id,
                client_username: Some(order.client_username),
                dispatcher_id: order.dispatcher_id,
                dispatcher_user_id: order.dispatcher_user_id,
                dispatcher_username: order.dispatcher_username,
                tow_truck_id: order.tow_truck_id,
                driver_user_id: order.driver_user_id,
                driver_username: order.driver_username,
                area_id: order.area_id,
                status: order.status,
                node_id: order.node_id,
                car_value: order.car_value,
                order_time: order.order_time,
                completed_time: order.completed_time,
            }
        }).collect();

        Ok(results)
    }

    pub async fn create_client_order(
        &self,
        client_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError> {
        match self
            .order_repository
            .create_order(client_id, node_id, car_value)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::BadRequest),
        }
    }

    pub async fn create_dispatcher_order(
        &self,
        order_id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
        order_time: DateTime<Utc>,
    ) -> Result<(), AppError> {
        if (self
            .order_repository
            .create_completed_order(order_id, tow_truck_id, order_time)
            .await)
            .is_err()
        {
            return Err(AppError::BadRequest);
        }

        self.order_repository
            .update_order_dispatched(order_id, dispatcher_id, tow_truck_id)
            .await?;

        self.tow_truck_repository
            .update_status(tow_truck_id, "busy")
            .await?;

        Ok(())
    }

    pub async fn get_completed_orders(&self) -> Result<Vec<CompletedOrderDto>, AppError> {
        let orders = self.order_repository.get_all_completed_orders().await?;
        let order_dtos = orders
            .into_iter()
            .map(CompletedOrderDto::from_entity)
            .collect();

        Ok(order_dtos)
    }
}
