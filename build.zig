const std = @import("std");

pub fn build(b: *std.Build) void {
    _ = b.addModule("zbinal", .{
        .root_source_file = b.path("src/module.zig"),
        .imports = &.{},
    });
}
