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
        feed_id -> Uuid,
        avito_ad_id -> Nullable<Varchar>,
        parsed_id -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        created_ts -> Timestamp,
    }
}

diesel::joinable!(avito_accounts -> users (user_id));
// Removed join between avito_ads and avito_accounts since account_id field was removed from avito_ads
diesel::joinable!(avito_ads -> avito_feeds (feed_id));  // Added relationship between ads and feeds
diesel::joinable!(avito_feeds -> avito_accounts (account_id));
diesel::joinable!(avito_requests -> users (user_id));

diesel::table! {
    avito_feeds (feed_id) {
        feed_id -> Uuid,
        account_id -> Uuid,
        category -> Varchar,
        created_ts -> Timestamptz,
        updated_ts -> Nullable<Timestamptz>,
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
diesel::table! {
    avito_ad_fields (field_id) {
        field_id -> Uuid,
        ad_id -> Uuid,
        tag -> Nullable<Varchar>,
        data_type -> Nullable<Varchar>,
        field_type -> Nullable<Varchar>,
        created_ts -> Timestamptz,
    }
}

diesel::table! {
    avito_ad_field_values (field_value_id) {
        field_value_id -> Uuid,
        field_id -> Nullable<Uuid>,
        value -> Nullable<Text>,
        created_ts -> Timestamptz,
    }
}

diesel::joinable!(avito_ad_field_values -> avito_ad_fields (field_id));
diesel::joinable!(avito_ad_fields -> avito_ads (ad_id));
diesel::joinable!(avito_request_progress -> avito_requests (request_id));

diesel::allow_tables_to_appear_in_same_query!(
	users,
	avito_accounts,
	avito_ads,
	avito_ad_fields,
	avito_ad_field_values,
	avito_analytics_ads,
	avito_feeds,
	avito_requests,
	avito_request_progress,
);
