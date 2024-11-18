use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header<'input> {
    From(&'input str),
    Date(DateTime<Utc>),
    Author(&'input str),
    Subject(&'input str),
    To(&'input str),
    Cc(&'input str),
    Bcc(&'input str),
    MessageId(&'input str),
    InReplyTo(&'input str),
    References(&'input str),
    ReplyTo(&'input str),
    Sender(&'input str),
    ReturnPath(&'input str),
    Received(&'input str),
    MimeVersion(&'input str),
    ContentType(&'input str),
    ContentTransferEncoding(&'input str),
    ContentId(&'input str),
    ContentDescription(&'input str),
    ContentDisposition(&'input str),
    ContentLanguage(&'input str),
    ContentLocation(&'input str),
    ContentMD5(&'input str),
    ContentLength(&'input str),
    Content(&'input str),
    Other(&'input str, &'input str),
}

impl<'input> TryFrom<(&'input str, &'input str)> for Header<'input> {
    type Error = String;

    fn try_from(value: (&'input str, &'input str)) -> Result<Self, Self::Error> {
        let (key, value) = value;
        let lkey = key.to_lowercase();

        match lkey.as_str() {
            "from" => Ok(Header::From(value)),
            "date" => Ok(
                Header::Date(
                    DateTime::parse_from_rfc2822(value)
                        .map_err(|e| e.to_string())
                        .map(|dt| dt.to_utc())?
                )
            ),
            "subject" => Ok(Header::Subject(value)),
            "to" => Ok(Header::To(value)),
            "cc" => Ok(Header::Cc(value)),
            "bcc" => Ok(Header::Bcc(value)),
            "message-id" => Ok(Header::MessageId(value)),
            "in-reply-to" => Ok(Header::InReplyTo(value)),
            "references" => Ok(Header::References(value)),
            "reply-to" => Ok(Header::ReplyTo(value)),
            "sender" => Ok(Header::Sender(value)),
            "return-path" => Ok(Header::ReturnPath(value)),
            "received" => Ok(Header::Received(value)),
            "mime-version" => Ok(Header::MimeVersion(value)),
            "content-type" => Ok(Header::ContentType(value)),
            "content-transfer-encoding" => Ok(Header::ContentTransferEncoding(value)),
            "content-id" => Ok(Header::ContentId(value)),
            "content-description" => Ok(Header::ContentDescription(value)),
            "content-disposition" => Ok(Header::ContentDisposition(value)),
            "content-language" => Ok(Header::ContentLanguage(value)),
            "content-location" => Ok(Header::ContentLocation(value)),
            "content-md5" => Ok(Header::ContentMD5(value)),
            "content-length" => Ok(Header::ContentLength(value)),
            "content" => Ok(Header::Content(value)),
            _ => Ok(Header::Other(key, value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header() {
        let header = Header::try_from(("Date", "Wed, 08 Jun 2022 12:00:01 -0300"));
        dbg!(&header);
        assert!(header.is_ok());
    }
}