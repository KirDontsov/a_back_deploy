// @generated automatically by Diesel CLI.

diesel::table! {
	users (id) {
		id -> Uuid,
		name -> Nullable<Varchar>,
		email -> Varchar,
		password -> Varchar,
		role -> Nullable<Varchar>,
		photo -> Nullable<Varchar>,
		verified -> Nullable<Bool>,
		favourite -> Array<Nullable<Text>>,
		created_at -> Nullable<Timestamp>,
		updated_at -> Nullable<Timestamp>,
	}
}

diesel::table! {
	avito_accounts (account_id) {
		account_id -> Uuid,
		user_id -> Uuid,
		client_id -> Varchar,
		avito_client_secret -> Text,
		avito_client_id -> Text,
		is_connected -> Nullable<Bool>,
		created_ts -> Timestamp,
		updated_ts -> Timestamp,
	}
}

diesel::table! {
	avito_ads (ad_id) {
		ad_id -> Uuid,
		account_id -> Uuid,
		title -> Varchar,
		description -> Nullable<Text>,
		price -> Nullable<Integer>,
		status -> Nullable<Varchar>,
		created_ts -> Timestamp,
		updated_ts -> Timestamp,
	}
}

diesel::joinable!(avito_accounts -> users (user_id));
diesel::joinable!(avito_ads -> avito_accounts (account_id));
diesel::joinable!(avito_feeds -> avito_accounts (account_id));
diesel::joinable!(avito_requests -> users (user_id));

diesel::table! {
	avito_feeds (feed_id) {
		feed_id -> Uuid,
		account_id -> Uuid,
		name -> Varchar,
		description -> Nullable<Text>,
		feed_type -> Nullable<Varchar>,
		is_active -> Nullable<Bool>,
		created_ts -> Timestamp,
		updated_ts -> Timestamp,
	}
}

diesel::table! {
	avito_requests (request_id) {
		request_id -> Uuid,
		request -> Text,
		city -> Nullable<Text>,
		coords -> Nullable<Text>,
		radius -> Nullable<Integer>,
		district -> Nullable<Text>,
		created_ts -> Timestamp,
		updated_ts -> Nullable<Timestamp>,
		user_id -> Uuid,
	}
}

diesel::table! {
	avito_request_progress (progress_id) {
		progress_id -> Uuid,
		request_id -> Uuid,
		progress -> Double,
		status -> Varchar,
		message -> Text,
		total_ads -> Integer,
		current_ads -> Integer,
		created_ts -> Timestamp,
		updated_ts -> Timestamp,
	}
}

diesel::table! {
	avito_analytics_ads (ad_id) {
		ad_id -> Uuid,
		my_ad -> Nullable<Varchar>,
		run_date -> Nullable<Timestamptz>,
		city_query -> Nullable<Varchar>,
		search_query -> Nullable<Varchar>,
		position -> Nullable<Integer>,
		views -> Nullable<Varchar>,
		views_today -> Nullable<Varchar>,
		promotion -> Nullable<Text>,
		delivery -> Nullable<Varchar>,
		ad_date -> Nullable<Varchar>,
		avito_ad_id -> Varchar,
		title -> Nullable<Text>,
		price -> Nullable<Varchar>,
		link -> Nullable<Text>,
		categories -> Nullable<Text>,
		seller_id -> Nullable<Varchar>,
		seller_name -> Nullable<Text>,
		seller_type -> Nullable<Varchar>,
		register_date -> Nullable<Varchar>,
		answer_time -> Nullable<Varchar>,
		rating -> Nullable<Varchar>,
		reviews_count -> Nullable<Varchar>,
		ads_count -> Nullable<Varchar>,
		closed_ads_count -> Nullable<Varchar>,
		photo_count -> Nullable<Varchar>,
		address -> Nullable<Text>,
		description -> Nullable<Text>,
		avito_request_id -> Nullable<Uuid>,
		created_ts -> Nullable<Timestamptz>,
	}
}
diesel::joinable!(avito_request_progress -> avito_requests (request_id));

diesel::allow_tables_to_appear_in_same_query!(
	users,
	avito_accounts,
	avito_ads,
	avito_analytics_ads,
	avito_feeds,
	avito_requests,
	avito_request_progress,
);
