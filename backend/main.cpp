#include "stdafx.h"

using namespace drogon;

int main()
{
    app().loadConfigFile("./config.json").run();
    return 0;
}