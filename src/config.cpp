#include "config.hpp"

#include <string>
#include <fstream>
#include <filesystem>

#include "crow.h"
/*
auto rv = crow::json::load(data.dump());
crow::json::wvalue wv{rv};
crow::mustache::context x{wv};
*/
using namespace std;

config_t default_config() {
    return config_t{
        8080,
        true,
        "files",
        "routes",
        "ssecret"
    };
}

void load_config(config_t& config)
{
    string config_str;
    ifstream config_file("config.json");

    if (config_file.is_open())
    {
        string temp;

        while (getline (config_file, temp)) {
            config_str += temp;
        }
        
        config_file.close();
    } else {
        return;
    }

    // auto config_json = ...
    crow::json::rvalue config_json = crow::json::load(config_str);

    if(
        !config_json.has("port") ||
        !config_json.has("console_log") ||
        !config_json.has("files") ||
        !config_json.has("routes") ||
        !config_json.has("secret")
    ) {
        return;
    }

    config.port = config_json["port"].u();
    config.console_log = config_json["console_log"].b();
    config.files = config_json["files"].s();
    config.routes = config_json["routes"].s();
    config.secret = config_json["secret"].s();
}