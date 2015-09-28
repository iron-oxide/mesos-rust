#[cfg(test)]
mod tests {

    use ::native::mesos_c::ProtobufObj;
    use ::proto;
    use protobuf::Message;

    #[test]
    fn protobufobj_message_translation() {
        let mut fi = proto::FrameworkInfo::new();
        fi.set_name("foo".to_string());
        fi.set_user("bar".to_string());

        let pb_data = &mut vec![];
        let pb = ProtobufObj::from_message(&fi, pb_data);

        let mut fi2 = proto::FrameworkInfo::new();
        fi2.merge_from_bytes(pb.to_bytes()).unwrap();

        assert_eq!(fi, fi2);
    }
}

