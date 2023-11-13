if eval "t.FORGED_RECIPIENTS && t.MAILLIST" {
	let "t.FORGED_RECIPIENTS_MAILLIST" "1";
}

if eval "t.FORGED_SENDER && t.MAILLIST" {
	let "t.FORGED_SENDER_MAILLIST" "1";
}

if eval "t.DMARC_POLICY_ALLOW && (t.SPF_SOFTFAIL || t.SPF_FAIL || t.DKIM_REJECT)" {
	let "t.DMARC_POLICY_ALLOW_WITH_FAILURES" "1";
}

if eval "t.DKIM_NA && t.SPF_NA && t.DMARC_NA && t.ARC_NA" {
	let "t.AUTH_NA" "1";
}

if eval "!(t.DKIM_NA && t.SPF_NA && t.DMARC_NA && t.ARC_NA) && (t.DKIM_NA || t.DKIM_TEMPFAIL || t.DKIM_PERMFAIL) && (t.SPF_NA || t.SPF_DNSFAIL) && t.DMARC_NA && (t.ARC_NA || t.ARC_DNSFAIL)" {
	let "t.AUTH_NA_OR_FAIL" "1";
}

if eval "(t.AUTH_NA || t.AUTH_NA_OR_FAIL) && (t.BOUNCE || t.SUBJ_BOUNCE_WORDS)" {
	let "t.BOUNCE_NO_AUTH" "1";
}

if eval "(t.HAS_X_POS || t.HAS_PHPMAILER_SIG) && t.HAS_WP_URI && (t.PHISHING || t.CRACKED_SURBL || t.PH_SURBL_MULTI || t.DBL_PHISH || t.DBL_ABUSE_PHISH || t.URIBL_BLACK || t.PHISHED_OPENPHISH || t.PHISHED_PHISHTANK)" {
	let "t.HACKED_WP_PHISHING" "1";
}

if eval "(t.HAS_XOIP || t.RCVD_FROM_SMTP_AUTH) && t.DCC_BULK" {
	let "t.COMPROMISED_ACCT_BULK" "1";
}

if eval "t.DCC_BULK && (t.MISSING_TO || t.R_UNDISC_RCPT)" {
	let "t.UNDISC_RCPTS_BULK" "1";
}

if eval "t.RECEIVED_SPAMHAUS_PBL && !t.RCVD_VIA_SMTP_AUTH" {
	let "t.RCVD_UNAUTH_PBL" "1";
}

if eval "(t.DKIM_ALLOW || t.ARC_ALLOW) && t.RCVD_IN_DNSWL_MED" {
	let "t.RCVD_DKIM_ARC_DNSWL_MED" "1";
}

if eval "(t.DKIM_ALLOW || t.ARC_ALLOW) && t.RCVD_IN_DNSWL_HI" {
	let "t.RCVD_DKIM_ARC_DNSWL_HI" "1";
}

if eval "(t.HAS_X_POS || t.HAS_PHPMAILER_SIG || t.HAS_X_PHP_SCRIPT) && (t.SUBJECT_ENDS_QUESTION || t.SUBJECT_ENDS_EXCLAIM || t.MANY_INVISIBLE_PARTS)" {
	let "t.AUTOGEN_PHP_SPAMMY" "1";
}

if eval "(t.PHISHING || t.DBL_PHISH || t.PHISHED_OPENPHISH || t.PHISHED_PHISHTANK) && (t.SUBJECT_ENDS_QUESTION || t.SUBJECT_ENDS_EXCLAIM)" {
	let "t.PHISH_EMOTION" "1";
}

if eval "t.HAS_GUC_PROXY_URI || t.URIBL_RED || t.DBL_ABUSE_REDIR || t.HAS_ONION_URI" {
	let "t.HAS_ANON_DOMAIN" "1";
}

if eval "(t.SPF_FAIL || t.SPF_SOFTFAIL) && (t.RCVD_COUNT_ZERO || t.RCVD_NO_TLS_LAST)" {
	let "t.VIOLATED_DIRECT_SPF" "1";
}

if eval "(t.FREEMAIL_FROM || t.FREEMAIL_ENVFROM || t.FREEMAIL_REPLYTO) && (t.TO_DN_RECIPIENTS || t.R_UNDISC_RCPT) && (t.FROM_NAME_HAS_TITLE || t.FREEMAIL_REPLYTO_NEQ_FROM_DOM)" {
	let "t.FREEMAIL_AFF" "1";
}

if eval "t.URL_ONLY && t.REDIRECTOR_URL" {
	let "t.REDIRECTOR_URL_ONLY" "1";
}

if eval "t.FAKE_REPLY && t.RCVD_VIA_SMTP_AUTH && (!t.RECEIVED_SPAMHAUS_PBL || t.RECEIVED_SPAMHAUS_XBL || t.RECEIVED_SPAMHAUS_SBL)" {
	let "t.THREAD_HIJACKING_FROM_INJECTOR" "1";
}
