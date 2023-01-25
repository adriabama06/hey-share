#include <iostream>
#include <string>
#include <map>
#include <thread>

#include "files.hpp"
#include "config.hpp"

#include "crow.h"

using namespace std;

config_t CONFIG = default_config();

map<std::string, std::string> ROUTES;

bool routes_reload_keep = true;
void routes_reload()
{
    while(routes_reload_keep)
    {
        std::this_thread::sleep_for(1000ms);
        files::load_all(CONFIG.routes, ROUTES);
    }
}

void exit_handler(int s)
{
    cout << "Caught signal" << s << endl;
}

int main(int argc, const char** argv)
{
    {
        struct sigaction sigIntHandler;

        sigIntHandler.sa_handler = exit_handler;
        sigemptyset(&sigIntHandler.sa_mask);
        sigIntHandler.sa_flags = 0;

        sigaction(SIGINT, &sigIntHandler, NULL);
    }

    load_config(CONFIG);
    files::load_all(CONFIG.routes, ROUTES);

    std::thread routes_reload_thread(routes_reload);

    for(auto it = ROUTES.cbegin(); it != ROUTES.cend(); ++it)
    {
        cout << it->first << " -> " << it->second << endl;
    }

    crow::SimpleApp app;

    if(!CONFIG.console_log)
    {
        app.loglevel(crow::LogLevel::CRITICAL);
    }

    CROW_CATCHALL_ROUTE(app)
    ([](const crow::request& req, crow::response& res) {
        if (res.code == 404)
        {
            res.set_static_file_info("404.html");
        }
        res.end();
    });

    CROW_ROUTE(app, "/<string>")
    ([](const crow::request& req, crow::response& res, string path) {
        map<string, string>::iterator it;

        it = ROUTES.find(path);

        if(it == ROUTES.end()) // not found
        {
            res.code = 404;
            return;
        }

        std::filesystem::path folder_path(CONFIG.files);
        std::filesystem::path file_path(it->second);
        std::filesystem::path full_path = folder_path / file_path;

        res.set_static_file_info(full_path.string());
        res.set_header("Content-Type", "application/octet-stream");
        res.set_header("Content-Disposition", "attachment;filename=" + it->second);
        res.end();
    });

    cout << "Starting at: http://localhost:" << CONFIG.port << endl;

    app.port(CONFIG.port)
    .multithreaded()
    .run();

    routes_reload_thread.join();

    return 0;
}