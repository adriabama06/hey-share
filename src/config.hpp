#ifndef CONFIG_H_
#define CONFIG_H_

#include <string>

typedef struct config_t
{
    unsigned short port;
    bool console_log;
    std::string files;
    std::string routes;
    std::string secret;
} config_t;

config_t default_config();

void load_config(config_t& config);

#endif