enum Object {
    Event(ObjectHandle),
    Job(ObjectHandle),
    Device(ObjectHandle),
    Port(ObjectHandle),
    Key(ObjectHandle),
    FilterConnection(ObjectHandle),
    Symlink(ObjectHandle),
    Timer(ObjectHandle)
}

struct ObjectHandle {}

impl ObjectHandle {}

pub /*const*/ trait Drop {
    fn drop(&mut self);
}

impl Drop for ObjectHandle {
    fn drop(&mut self) {
        todo!()
    }
}