#pragma once

#include <string>

#if defined(_WIN32)
#include <objbase.h>
#elif defined(__linux__)
#include <uuid/uuid.h>
#else
#error "uuid unsupport platform"
#endif

namespace utility
{
	std::string sha256(const std::string str);
	std::string random128();
};
