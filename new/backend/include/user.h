#pragma once

#include <drogon/HttpController.h>

using namespace drogon;

namespace api
{
	class User : public HttpController<User>
	{
	public:
		METHOD_LIST_BEGIN
			METHOD_ADD(User::signup, "/signup", Options, Post, "api::LogFilter");
		METHOD_ADD(User::signin, "/signin", Options, Post, "api::LogFilter");
		METHOD_ADD(User::signout, "/signout", Options, Post, "api::JwtFilter");
		METHOD_ADD(User::info, "/info/{id}", Get, "api::JwtFilter");
		METHOD_ADD(User::update, "/update", Options, Post, "api::JwtFilter");
		METHOD_LIST_END

	public:
		void signup(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void signin(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void signout(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback, const std::string& id) const;

		void update(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;
	};
}
