// Arcturus - Hobbyist operating system written in Zig.
// Copyright (C) 2025 Theomund
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

const std = @import("std");

pub fn build(b: *std.Build) void {
    var target_query: std.Target.Query = .{
        .cpu_arch = .x86_64,
        .os_tag = .freestanding,
        .abi = .none,
    };

    const Feature = std.Target.x86.Feature;
    target_query.cpu_features_add.addFeature(@intFromEnum(Feature.soft_float));
    target_query.cpu_features_sub.addFeature(@intFromEnum(Feature.mmx));
    target_query.cpu_features_sub.addFeature(@intFromEnum(Feature.sse));
    target_query.cpu_features_sub.addFeature(@intFromEnum(Feature.sse2));
    target_query.cpu_features_sub.addFeature(@intFromEnum(Feature.avx));
    target_query.cpu_features_sub.addFeature(@intFromEnum(Feature.avx2));

    const kernel_main_path = b.path("src/kernel/main.zig");
    const linker_script_path = b.path("src/kernel/linker.ld");
    const target = b.resolveTargetQuery(target_query);
    const optimize = b.standardOptimizeOption(.{});

    const kernel = b.addExecutable(.{
        .name = "kernel",
        .root_source_file = kernel_main_path,
        .target = target,
        .optimize = optimize,
        .code_model = .kernel,
    });

    kernel.want_lto = false;
    kernel.setLinkerScriptPath(linker_script_path);

    b.installArtifact(kernel);

    const zig_clean_cmd = b.addSystemCommand(&.{ "rm", "-rf", b.install_path });

    const clean_step = b.step("clean", "Clean the project");
    clean_step.dependOn(&zig_clean_cmd.step);

    const zig_format_cmd = b.addSystemCommand(&.{ "zig", "fmt", "." });

    const format_step = b.step("format", "Format the source code");
    format_step.dependOn(&zig_format_cmd.step);

    const vale_sync_cmd = b.addSystemCommand(&.{ "vale", "sync" });
    const vale_lint_cmd = b.addSystemCommand(&.{ "vale", "README.md" });
    vale_lint_cmd.step.dependOn(&vale_sync_cmd.step);

    const yaml_lint_cmd = b.addSystemCommand(&.{ "yamllint", ".github/workflows" });

    const lint_step = b.step("lint", "Run the project linters");
    lint_step.dependOn(&vale_lint_cmd.step);
    lint_step.dependOn(&yaml_lint_cmd.step);

    const kernel_root_path = b.path("src/kernel/root.zig");

    const unit_tests = b.addTest(.{
        .name = "test",
        .root_source_file = kernel_root_path,
        .optimize = optimize,
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);

    const test_step = b.step("test", "Run the unit test suite");
    test_step.dependOn(&run_unit_tests.step);
}
