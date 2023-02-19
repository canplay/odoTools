#pragma once

#include <drogon/HttpController.h>

using namespace drogon;

namespace api
{
	class User : public HttpController<User>
	{
	public:
		METHOD_LIST_BEGIN
			METHOD_ADD(User::count, "/count", Options, Get, "api::JwtFilter");
		METHOD_ADD(User::signup, "/signup", Options, Post, "api::LogFilter");
		METHOD_ADD(User::signin, "/signin", Options, Post, "api::LogFilter");
		METHOD_ADD(User::signout, "/signout", Options, Post, "api::JwtFilter");
		METHOD_ADD(User::info, "/info", Options, Post, "api::JwtFilter");
		METHOD_ADD(User::infoOne, "/info/{id}", Options, Get, "api::JwtFilter");
		METHOD_ADD(User::update, "/update", Options, Post, "api::JwtFilter");
		METHOD_LIST_END

	public:
		void count(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void signup(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void signin(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void signout(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void infoOne(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback, const std::string& id) const;

		void update(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;
	};
}
