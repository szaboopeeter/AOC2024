const std = @import("std");

// Example input:
// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3

pub fn main() !void {
    var file = try std.fs.cwd().openFile("src/input.txt", .{});
    defer file.close();
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();
    var buf: [20]u8 = undefined;
    var is_first = true;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var first = std.ArrayList(i32).init(allocator);
    var second = std.ArrayList(i32).init(allocator);

    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var parts = std.mem.splitScalar(u8, line, ' ');
        while (parts.next()) |part| {
            if (std.fmt.parseInt(i32, part, 10)) |val| {
                if (is_first) {
                    first.append(val) catch |err| {
                        std.debug.print("Error: {}\n", .{err});
                        return err;
                    };
                } else {
                    second.append(val) catch |err| {
                        std.debug.print("Error: {}\n", .{err});
                        return err;
                    };
                }
                is_first = !is_first;
            } else |_| {}
        }
    }
    std.mem.sort(i32, first.items, {}, std.sort.asc(i32));
    std.mem.sort(i32, second.items, {}, std.sort.asc(i32));

    var accum: u32 = 0;
    for (first.items, second.items) |fi, si| {
        const diff = @abs(fi - si);
        accum += diff;
    }
    std.debug.print("{}\n", .{accum});
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
