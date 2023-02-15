#pragma once

#include <drogon/HttpController.h>

using namespace drogon;

namespace api
{
	class Statistics : public HttpController<Statistics>
	{
	public:
		METHOD_LIST_BEGIN
			METHOD_ADD(Statistics::formal_count, "/formal/count", Options, Post, "api::JwtFilter");
		METHOD_ADD(Statistics::formal_info, "/formal/info", Options, Post, "api::JwtFilter");
		METHOD_ADD(Statistics::formal_total, "/formal/total", Options, Post, "api::JwtFilter");
		METHOD_ADD(Statistics::payment_info_count, "/payment_info/count", Options, Post, "api::JwtFilter");
		METHOD_ADD(Statistics::payment_info_info, "/payment_info/info", Options, Post, "api::JwtFilter");
		METHOD_ADD(Statistics::payment_info_total, "/payment_info/total", Options, Post, "api::JwtFilter");
		METHOD_ADD(Statistics::payment_web_count, "/payment_web/count", Options, Post, "api::JwtFilter");
		METHOD_ADD(Statistics::payment_web_info, "/payment_web/info", Options, Post, "api::JwtFilter");
		METHOD_ADD(Statistics::payment_web_total, "/payment_web/total", Options, Post, "api::JwtFilter");
		METHOD_LIST_END

	public:
		void formal_count(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void formal_info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void formal_total(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void payment_info_count(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void payment_info_info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void payment_info_total(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void payment_web_count(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void payment_web_info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void payment_web_total(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;
	};
}