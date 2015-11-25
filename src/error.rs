//! Error struct and methods

use std::result;
use std::error;
use std::io;
use std::fmt;

use num::traits::FromPrimitive;
use byteorder;


/// A type for results generated by this crate's functions where the `Err` type
/// is hard-wired to `enums::Error`.
///
/// This typedef is generally used to avoid writing out `enums::Error` directly and
/// is otherwise a direct mapping to `std::result::Result`.
pub type Result<T> = result::Result<T, Error>;

/// The various errors this library can produce.
#[derive(Debug)]
pub enum Error {
    /// Input/Output error while communicating with Kafka
    Io(io::Error),
    /// An error as reported by a remote Kafka server
    Kafka(KafkaCode),

    /// Failure to decode a snappy compressed response from Kafka
    InvalidInputSnappy,
    /// Failure to decode a response due to an insufficient number of bytes available
    UnexpectedEOF,
    /// Failure to decode or encode a response or request respectively
    CodecError,
    /// Failure to decode a string into a valid utf8 byte sequence
    StringDecodeError,
    /// Unable to reach any host
    NoHostReachable,
}

/// Various errors reported by a remote Kafka server.
/// See also [Kafka Errors](https://cwiki.apache.org/confluence/display/KAFKA/A+Guide+To+The+Kafka+Protocol#AGuideToTheKafkaProtocol-ErrorCodes)
#[derive(Debug)]
pub enum KafkaCode {
    /// An unexpected server error
    Unknown,
    /// The requested offset is outside the range of offsets
    /// maintained by the server for the given topic/partition
    OffsetOutOfRange,
    /// This indicates that a message contents does not match its CRC
    InvalidMessage,
    /// This request is for a topic or partition that does not exist
    /// on this broker.
    UnknownTopicOrPartition,
    /// The message has a negative size
    InvalidMessageSize,
    /// This error is thrown if we are in the middle of a leadership
    /// election and there is currently no leader for this partition
    /// and hence it is unavailable for writes.
    LeaderNotAvailable,
    /// This error is thrown if the client attempts to send messages
    /// to a replica that is not the leader for some partition. It
    /// indicates that the clients metadata is out of date.
    NotLeaderForPartition,
    /// This error is thrown if the request exceeds the user-specified
    /// time limit in the request.
    RequestTimedOut,
    /// This is not a client facing error and is used mostly by tools
    /// when a broker is not alive.
    BrokerNotAvailable,
    /// If replica is expected on a broker, but is not (this can be
    /// safely ignored).
    ReplicaNotAvailable,
    /// The server has a configurable maximum message size to avoid
    /// unbounded memory allocation. This error is thrown if the
    /// client attempt to produce a message larger than this maximum.
    MessageSizeTooLarge,
    /// Internal error code for broker-to-broker communication.
    StaleControllerEpochCode,
    /// If you specify a string larger than configured maximum for
    /// offset metadata
    OffsetMetadataTooLargeCode,
    /// The broker returns this error code for an offset fetch request
    /// if it is still loading offsets (after a leader change for that
    /// offsets topic partition), or in response to group membership
    /// requests (such as heartbeats) when group metadata is being
    /// loaded by the coordinator.
    OffsetsLoadInProgressCode,
    /// The broker returns this error code for group coordinator
    /// requests, offset commits, and most group management requests
    /// if the offsets topic has not yet been created, or if the group
    /// coordinator is not active.
    ConsumerCoordinatorNotAvailableCode,
    /// The broker returns this error code if it receives an offset
    /// fetch or commit request for a group that it is not a
    /// coordinator for.
    NotCoordinatorForConsumerCode,
    /// For a request which attempts to access an invalid topic
    /// (e.g. one which has an illegal name), or if an attempt is made
    /// to write to an internal topic (such as the consumer offsets
    /// topic).
    InvalidTopicCode,
    /// If a message batch in a produce request exceeds the maximum
    /// configured segment size.
    RecordListTooLargeCode,
    /// Returned from a produce request when the number of in-sync
    /// replicas is lower than the configured minimum and requiredAcks is
    /// -1.
    NotEnoughReplicasCode,
    /// Returned from a produce request when the message was written
    /// to the log, but with fewer in-sync replicas than required.
    NotEnoughReplicasAfterAppendCode,
    /// Returned from a produce request if the requested requiredAcks is
    /// invalid (anything other than -1, 1, or 0).
    InvalidRequiredAcksCode,
    /// Returned from group membership requests (such as heartbeats) when
    /// the generation id provided in the request is not the current
    /// generation.
    IllegalGenerationCode,
    /// Returned in join group when the member provides a protocol type or
    /// set of protocols which is not compatible with the current group.
    InconsistentGroupProtocolCode,
    /// Returned in join group when the groupId is empty or null.
    InvalidGroupIdCode,
    /// Returned from group requests (offset commits/fetches, heartbeats,
    /// etc) when the memberId is not in the current generation.
    UnknownMemberIdCode,
    /// Return in join group when the requested session timeout is outside
    /// of the allowed range on the broker
    InvalidSessionTimeoutCode,
    /// Returned in heartbeat requests when the coordinator has begun
    /// rebalancing the group. This indicates to the client that it
    /// should rejoin the group.
    RebalanceInProgressCode,
    /// This error indicates that an offset commit was rejected because of
    /// oversize metadata.
    InvalidCommitOffsetSizeCode,
    /// Returned by the broker when the client is not authorized to access
    /// the requested topic.
    TopicAuthorizationFailedCode,
    /// Returned by the broker when the client is not authorized to access
    /// a particular groupId.
    GroupAuthorizationFailedCode,
    /// Returned by the broker when the client is not authorized to use an
    /// inter-broker or administrative API.
    ClusterAuthorizationFailedCode,
}

// XXX is it really necessary we do implement `FromPrimitive`?
impl FromPrimitive for Error {
    fn from_i16(n: i16) -> Option<Error> {
        match n {
            1 => Some(Error::Kafka(KafkaCode::OffsetOutOfRange)),
            2 => Some(Error::Kafka(KafkaCode::InvalidMessage)),
            3 => Some(Error::Kafka(KafkaCode::UnknownTopicOrPartition)),
            4 => Some(Error::Kafka(KafkaCode::InvalidMessageSize)),
            5 => Some(Error::Kafka(KafkaCode::LeaderNotAvailable)),
            6 => Some(Error::Kafka(KafkaCode::NotLeaderForPartition)),
            7 => Some(Error::Kafka(KafkaCode::RequestTimedOut)),
            8 => Some(Error::Kafka(KafkaCode::BrokerNotAvailable)),
            9 => Some(Error::Kafka(KafkaCode::ReplicaNotAvailable)),
            10 => Some(Error::Kafka(KafkaCode::MessageSizeTooLarge)),
            11 => Some(Error::Kafka(KafkaCode::StaleControllerEpochCode)),
            12 => Some(Error::Kafka(KafkaCode::OffsetMetadataTooLargeCode)),
            14 => Some(Error::Kafka(KafkaCode::OffsetsLoadInProgressCode)),
            15 => Some(Error::Kafka(KafkaCode::ConsumerCoordinatorNotAvailableCode)),
            16 => Some(Error::Kafka(KafkaCode::NotCoordinatorForConsumerCode)),
            17 => Some(Error::Kafka(KafkaCode::InvalidTopicCode)),
            18 => Some(Error::Kafka(KafkaCode::RecordListTooLargeCode)),
            19 => Some(Error::Kafka(KafkaCode::NotEnoughReplicasCode)),
            20 => Some(Error::Kafka(KafkaCode::NotEnoughReplicasAfterAppendCode)),
            21 => Some(Error::Kafka(KafkaCode::InvalidRequiredAcksCode)),
            22 => Some(Error::Kafka(KafkaCode::IllegalGenerationCode)),
            23 => Some(Error::Kafka(KafkaCode::InconsistentGroupProtocolCode)),
            24 => Some(Error::Kafka(KafkaCode::InvalidGroupIdCode)),
            25 => Some(Error::Kafka(KafkaCode::UnknownMemberIdCode)),
            26 => Some(Error::Kafka(KafkaCode::InvalidSessionTimeoutCode)),
            27 => Some(Error::Kafka(KafkaCode::RebalanceInProgressCode)),
            28 => Some(Error::Kafka(KafkaCode::InvalidCommitOffsetSizeCode)),
            29 => Some(Error::Kafka(KafkaCode::TopicAuthorizationFailedCode)),
            30 => Some(Error::Kafka(KafkaCode::GroupAuthorizationFailedCode)),
            31 => Some(Error::Kafka(KafkaCode::ClusterAuthorizationFailedCode)),
            -1 => Some(Error::Kafka(KafkaCode::Unknown)),
            _ => None
        }
    }
    fn from_i64(_: i64) -> Option<Error> {
        Some(Error::Kafka(KafkaCode::Unknown))
    }
    fn from_u64(_: u64) -> Option<Error> {
        Some(Error::Kafka(KafkaCode::Unknown))
    }
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}


impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            byteorder::Error::UnexpectedEOF => Error::UnexpectedEOF,
            byteorder::Error::Io(err) => Error::Io(err)
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        match *self {
            Error::Io(ref err) => Error::Io(io::Error::new(err.kind(), "Io Error")),
            ref x => x.clone()
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => error::Error::description(err),
            Error::Kafka(_) => "Kafka Error",
            Error::InvalidInputSnappy => "Snappy decode error",
            Error::UnexpectedEOF => "Unexpected EOF",
            Error::CodecError => "Encoding/Decoding error",
            Error::StringDecodeError => "String decoding error",
            Error::NoHostReachable => "No host reachable",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => err.cause(),
            _ => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Kafka(ref c) => write!(f, "Kafka Error ({:?})", c),
            Error::InvalidInputSnappy => write!(f, "{}", "Snappy decoding error"),
            Error::UnexpectedEOF => write!(f, "Unexpected EOF"),
            Error::CodecError => write!(f, "Encoding/Decoding Error"),
            // XXX might want to provide some context about parsed string and the error position with in
            Error::StringDecodeError => write!(f, "String decoding error"),
            Error::NoHostReachable => write!(f, "No Host Reachable"),
        }
    }
}
