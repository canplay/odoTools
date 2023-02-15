#include "stdafx.h"
#include "jwt.h"

namespace api
{
	jwt::jwt(const std::string& token, const std::int64_t& expiration)
	{
		this->token = token;
		this->expiration = expiration;
	}

	std::string jwt::getToken() const
	{
		return this->token;
	}

	std::int64_t jwt::getExpiration() const
	{
		return this->expiration;
	}

	jwt jwt::generateToken(const std::map<std::string, ::jwt::traits::kazuho_picojson::value_type>& claims, const bool& extension)
	{
		const auto time = std::chrono::system_clock::now();

		const int64_t expiresAt = std::chrono::duration_cast<std::chrono::seconds>((time + std::chrono::hours{ (extension ? 30 : 1) * 24 }).time_since_epoch()).count();

		auto jwtToken = ::jwt::create()
			.set_type("JWT")
			.set_issuer(drogon::app().getCustomConfig()["jwt"]["issuer"].asString())
			.set_audience(drogon::app().getCustomConfig()["jwt"]["audience"].asString())
			.set_issued_at(time)
			.set_not_before(time)
			.set_expires_at(std::chrono::system_clock::from_time_t(expiresAt));

		for (auto& claim : claims)
			jwtToken.set_payload_claim(claim.first, claim.second);

		return {
			jwtToken.sign(::jwt::algorithm::hs256{drogon::app().getCustomConfig()["jwt"]["private_key"].asString()}),
			expiresAt
		};
	}

	std::map<std::string, std::any> jwt::decodeToken(const std::string& encodedToken)
	{
		try
		{
			auto decodedToken = ::jwt::decode(encodedToken);

			if (verifyToken(decodedToken))
			{
				std::map<std::string, std::any> attributes = {};

				for (auto& claim : decodedToken.get_payload_claims())
					addClaimToAttributes(attributes, claim);

				return attributes;
			}

			return {};
		}
		catch (const std::exception& e)
		{
			LOG_ERROR << e.what();
			return {};
		}
	}

	bool jwt::verifyToken(const ::jwt::decoded_jwt<::jwt::traits::kazuho_picojson>& jwt)
	{
		auto jwtVerifier = ::jwt::verify()
			.with_issuer(drogon::app().getCustomConfig()["jwt"]["issuer"].asString())
			.with_audience(drogon::app().getCustomConfig()["jwt"]["audience"].asString())
			.allow_algorithm(::jwt::algorithm::hs256{ drogon::app().getCustomConfig()["jwt"]["private_key"].asString() });

		try
		{
			jwtVerifier.verify(jwt);
			return true;
		}
		catch (const ::jwt::token_verification_exception& e)
		{
			LOG_ERROR << e.what();
			return false;
		}
	}

	void jwt::addClaimToAttributes(std::map<std::string, std::any>& attributes, const std::pair<std::string, ::jwt::basic_claim<::jwt::traits::kazuho_picojson>>& claim)
	{
		switch (claim.second.get_type())
		{
		case ::jwt::json::type::boolean:
			attributes.insert(std::pair<std::string, bool>(claim.first, claim.second.as_bool()));
			break;
		case ::jwt::json::type::integer:
			attributes.insert(std::pair<std::string, std::int64_t>(claim.first, claim.second.as_int()));
			break;
		case ::jwt::json::type::number:
			attributes.insert(std::pair<std::string, double>(claim.first, claim.second.as_number()));
			break;
		case ::jwt::json::type::string:
			attributes.insert(std::pair<std::string, std::string>(claim.first, claim.second.as_string()));
			break;
		case ::jwt::json::type::array:
			attributes.insert(std::pair<std::string, std::any>(claim.first, claim.second.as_array()));
			break;
		case ::jwt::json::type::object:
			attributes.insert(std::pair<std::string, std::any>(claim.first, claim.second));
			break;
		default: throw std::bad_cast();
		}
	}
}