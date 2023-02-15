#pragma

#include <drogon/HttpController.h>

using namespace drogon;

namespace api
{
	class Order : public HttpController<Order>
	{
	public:
		METHOD_LIST_BEGIN
			METHOD_ADD(Order::count, "/count", Options, Post, "api::JwtFilter");
		METHOD_ADD(Order::info, "/info", Options, Post, "api::JwtFilter");
		METHOD_ADD(Order::update, "/update", Options, Post, "api::JwtFilter");
		METHOD_LIST_END

	public:
		void count(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void info(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

		void update(const HttpRequestPtr& req, std::function<void(const HttpResponsePtr&)>&& callback) const;

	private:
		Task<Json::Value> insert(const Json::Value json) const;
	};
}