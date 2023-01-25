#ifndef FILES_H_
#define FILES_H_

#include <map>
#include <string>

namespace files {
    void load(
        std::string& file,
        std::string& folder,
        std::map<std::string, std::string>& map
    );
    void load_all(std::string& folder, std::map<std::string, std::string>& map);
};

#endif