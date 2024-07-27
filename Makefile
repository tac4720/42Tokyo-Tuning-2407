ARCH := $(shell uname -m)
ifeq ($(ARCH), x86_64)
	ALP_ZIP = alp_linux_amd64.zip
else ifeq ($(ARCH), arm64)
	ALP_ZIP = alp_darwin_arm64.zip
endif
ALP_DOWNLOAD_URL = https://github.com/tkuchiki/alp/releases/download/v1.0.21/${ALP_ZIP}
PERF_DATA_DIR = perf_data
PT_QUERY_DIGEST_ZIP = v3.6.0.zip
PT_QUERY_DIGEST_DOWNLOAD_URL = https://github.com/percona/percona-toolkit/archive/refs/tags/${PT_QUERY_DIGEST_ZIP}

# prepare for performance measurement
init: init.alp init.pprof init.pt-query-digest

init.alp:
	curl -L -O $(ALP_DOWNLOAD_URL)
	unzip $(ALP_ZIP)
	rm $(ALP_ZIP)
	sudo mv alp /usr/local/bin/

init.pprof:
	go install github.com/google/pprof@latest

init.pt-query-digest:
	curl -L -O $(PT_QUERY_DIGEST_DOWNLOAD_URL)
	unzip $(PT_QUERY_DIGEST_ZIP)
	rm $(PT_QUERY_DIGEST_ZIP)
	sudo mv ./percona-toolkit-3.6.0/bin/pt-query-digest /usr/local/bin/pt-query-digest
	rm -rf ./percona-toolkit-3.6.0

truncate:
	rm $(PERF_DATA_DIR)/* || true
	truncate -s 0 ./webapp/nginx/log/access.log
	truncate -s 0 ./webapp/mysql/log/slow.log

# measure performance
measure: measure.alp
m: measure

measure.alp:
	cat ./webapp/nginx/log/access.log | alp json --limit 10000 --sort=sum --reverse --query-string --format=table -o count,method,uri,avg,p99,max --matching-groups '/api/user_image/\d+,/api/order/\d+,orders/\d+,/_next.+,__nextjs.+' > $(PERF_DATA_DIR)/alp.log

# TODO add pprof configuration to main.rs
measure.pprof:

measure.query:
	cat ./webapp/mysql/log/slow.log | pt-query-digest > $(PERF_DATA_DIR)/pt-query-digest.log

restart: restart.nginx restart.mysql

# restart nginx container to apply new configuration
restart.nginx:
	cd webapp && docker-compose -f docker-compose.local.yml restart nginx

restart.mysql:
	cd webapp && bash restart_container.sh
