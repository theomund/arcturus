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
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    clean(b);
    format(b);
    lint(b);
    unit(b, target, optimize);
    const kernel_artifact = kernel(b, optimize);
    const limine_artifact = limine(b, target, optimize);
    const iso_artifact = iso(b, kernel_artifact, limine_artifact);
    bios(b, iso_artifact);
    uefi(b, iso_artifact);
}

fn clean(b: *std.Build) void {
    const zig_clean = b.addRemoveDirTree(b.install_path);

    const clean_step = b.step("clean", "Clean the project");
    clean_step.dependOn(&zig_clean.step);
}

fn format(b: *std.Build) void {
    const zig_format = b.addFmt(.{ .paths = &.{"."} });

    const format_step = b.step("format", "Format the source code");
    format_step.dependOn(&zig_format.step);
}

fn lint(b: *std.Build) void {
    const vale_sync_cmd = b.addSystemCommand(&.{ "vale", "sync" });
    const vale_lint_cmd = b.addSystemCommand(&.{ "vale", "README.md" });
    vale_lint_cmd.step.dependOn(&vale_sync_cmd.step);

    const yaml_lint_cmd = b.addSystemCommand(&.{ "yamllint", ".github/workflows" });

    const lint_step = b.step("lint", "Run the project linters");
    lint_step.dependOn(&vale_lint_cmd.step);
    lint_step.dependOn(&yaml_lint_cmd.step);
}

fn unit(b: *std.Build, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) void {
    const unit_tests = b.addTest(.{
        .name = "unit",
        .root_source_file = b.path("src/main/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);

    const test_step = b.step("unit", "Run the unit test suite");
    test_step.dependOn(&run_unit_tests.step);
}

fn kernel(b: *std.Build, optimize: std.builtin.OptimizeMode) *std.Build.Step.InstallArtifact {
    var query: std.Target.Query = .{
        .cpu_arch = .x86_64,
        .os_tag = .freestanding,
        .abi = .none,
    };

    const Feature = std.Target.x86.Feature;
    query.cpu_features_add.addFeature(@intFromEnum(Feature.soft_float));
    query.cpu_features_sub.addFeature(@intFromEnum(Feature.mmx));
    query.cpu_features_sub.addFeature(@intFromEnum(Feature.sse));
    query.cpu_features_sub.addFeature(@intFromEnum(Feature.sse2));
    query.cpu_features_sub.addFeature(@intFromEnum(Feature.avx));
    query.cpu_features_sub.addFeature(@intFromEnum(Feature.avx2));

    const target = b.resolveTargetQuery(query);

    const executable = b.addExecutable(.{
        .name = "kernel",
        .root_source_file = b.path("src/kernel/main.zig"),
        .target = target,
        .optimize = optimize,
        .code_model = .kernel,
    });

    executable.want_lto = false;
    executable.setLinkerScriptPath(b.path("src/kernel/linker.ld"));

    const artifact = b.addInstallArtifact(executable, .{
        .dest_dir = .{
            .override = .{ .custom = "iso/boot/" },
        },
    });

    const kernel_step = b.step("kernel", "Build the kernel");
    kernel_step.dependOn(&artifact.step);

    return artifact;
}

fn limine(b: *std.Build, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) *std.Build.Step.Run {
    const executable = b.addExecutable(.{
        .name = "limine",
        .target = target,
        .optimize = optimize,
    });

    const dependency = b.dependency("limine", .{});

    executable.addCSourceFile(.{ .file = dependency.path("limine.c") });
    executable.linkLibC();

    const artifact = b.addRunArtifact(executable);

    const limine_step = b.step("limine", "Build the bootloader");
    limine_step.dependOn(&artifact.step);

    return artifact;
}

fn iso(b: *std.Build, kernel_artifact: *std.Build.Step.InstallArtifact, limine_artifact: *std.Build.Step.Run) *std.Build.Step.InstallFile {
    const dependency = b.dependency("limine", .{});

    const boot_ia32 = b.addInstallFile(dependency.path("BOOTIA32.EFI"), "iso/EFI/BOOT/BOOTIA32.EFI");
    const boot_x64 = b.addInstallFile(dependency.path("BOOTX64.EFI"), "iso/EFI/BOOT/BOOTX64.EFI");
    const limine_bios_cd = b.addInstallFile(dependency.path("limine-bios-cd.bin"), "iso/boot/limine/limine-bios-cd.bin");
    const limine_bios_sys = b.addInstallFile(dependency.path("limine-bios.sys"), "iso/boot/limine/limine-bios.sys");
    const limine_config = b.addInstallFile(b.path("src/bootloader/limine.conf"), "iso/boot/limine/limine.conf");
    const limine_uefi_cd = b.addInstallFile(dependency.path("limine-uefi-cd.bin"), "iso/boot/limine/limine-uefi-cd.bin");

    const xorriso = b.addSystemCommand(&.{
        "xorriso",
        "-as",
        "mkisofs",
        "-R",
        "-r",
        "-J",
        "-b",
        "boot/limine/limine-bios-cd.bin",
        "-no-emul-boot",
        "-boot-load-size",
        "4",
        "-boot-info-table",
        "-hfsplus",
        "-apm-block-size",
        "2048",
        "--efi-boot",
        "boot/limine/limine-uefi-cd.bin",
        "-efi-boot-part",
        "--efi-boot-image",
        "--protective-msdos-label",
        "-o",
    });
    const iso_file = xorriso.addOutputFileArg("arcturus.iso");
    xorriso.addDirectoryArg(b.path("zig-out/iso"));
    xorriso.step.dependOn(&boot_ia32.step);
    xorriso.step.dependOn(&boot_x64.step);
    xorriso.step.dependOn(&kernel_artifact.step);
    xorriso.step.dependOn(&limine_bios_cd.step);
    xorriso.step.dependOn(&limine_bios_sys.step);
    xorriso.step.dependOn(&limine_config.step);
    xorriso.step.dependOn(&limine_uefi_cd.step);

    limine_artifact.addArg("bios-install");
    limine_artifact.addFileArg(iso_file);

    const artifact = b.addInstallFile(iso_file, "bin/arcturus.iso");
    artifact.step.dependOn(&limine_artifact.step);

    const iso_step = b.step("iso", "Build the ISO image");
    iso_step.dependOn(&artifact.step);

    return artifact;
}

fn bios(b: *std.Build, iso_artifact: *std.Build.Step.InstallFile) void {
    const qemu = b.addSystemCommand(&.{
        "qemu-system-x86_64",
        "-M",
        "q35",
        "-boot",
        "d",
        "-cdrom",
    });
    qemu.addFileArg(b.path("zig-out/bin/arcturus.iso"));
    qemu.step.dependOn(&iso_artifact.step);

    const bios_step = b.step("bios", "Create a BIOS virtual machine");
    bios_step.dependOn(&qemu.step);
}

fn uefi(b: *std.Build, iso_artifact: *std.Build.Step.InstallFile) void {
    const qemu = b.addSystemCommand(&.{
        "qemu-system-x86_64",
        "-M",
        "q35",
        "-drive",
        "if=pflash,unit=0,format=raw,file=/usr/share/OVMF/OVMF_CODE.fd,readonly=on",
        "-cdrom",
    });
    qemu.addFileArg(b.path("zig-out/bin/arcturus.iso"));
    qemu.step.dependOn(&iso_artifact.step);

    const uefi_step = b.step("uefi", "Create a UEFI virtual machine");
    uefi_step.dependOn(&qemu.step);
}
