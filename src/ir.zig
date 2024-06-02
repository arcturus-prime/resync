const std = @import("std");

const Code = enum(u8) {
    data = 0,

    resize,

    xor,
    @"or",
    @"and",
    not,
    shift,
    ishift,

    add,
    sub,
    mul,
    div,
    mod,
    neg,

    imul,
    idiv,

    fadd,
    fsub,
    fmul,
    fdiv,
    fneg,

    lt,
    lte,
    gt,
    gte,
    eq,

    load,
    store,

    @"return",

    call,
    ccall,
    cjump,
    jump,
};

const CodeInfo = [_]struct { usize, usize }{
    .{ 0, 1 },
    .{ 1, 1 },
};

const Data = union {
    u: usize,
    i: isize,
    f32: f32,
    f64: f64,
    str: [8]u8,
};

const Block = struct {
    code: std.ArrayListUnmanaged(Code),
    data: std.ArrayListUnmanaged(Data),
};
