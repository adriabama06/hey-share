#include "files.hpp"

#include <iostream>
#include <string>
#include <map>
#include <filesystem>

#include "crow.h"

void files::load(
    std::string& file,
    std::string& folder,
    std::map<std::string, std::string>& map
) {
    std::string route_str;

    std::filesystem::path folder_path(folder);
    std::filesystem::path file_path(file);
    std::filesystem::path full_path = folder_path / file_path;


    std::ifstream route_file(full_path);

    if (route_file.is_open())
    {
        std::string temp;

        while (getline (route_file, temp)) {
            route_str += temp;
        }

        route_file.close();
    } else {
        return;
    }

    // auto route_json = ...
    crow::json::rvalue route_json;

    try {
        route_json = crow::json::load(route_str);
    } catch (...) {
        return;
    }

    if(
        !route_json.has("url") ||
        !route_json.has("file")
    ) {
        return;
    }

    map[route_json["url"].s()] = route_json["file"].s();
}

void files::load_all(std::string& folder, std::map<std::string, std::string>& map)
{
    std::map<std::string, std::string> new_map;

    for(const auto& entry : std::filesystem::directory_iterator(folder))
    {
        std::string filename = entry.path().filename().string();
        files::load(filename, folder, new_map);
    }

    map = new_map;
}
