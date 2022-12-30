// Verifies that pointer type membership tests for indirect calls are emitted.
//
// needs-sanitizer-cfi
// compile-flags: -Clto -Cno-prepopulate-passes -Ctarget-feature=-crt-static -Zsanitizer=cfi

#![crate_type="lib"]

pub fn foo(f: fn(i32) -> i32, arg: i32) -> i32 {
    // CHECK-LABEL: define{{.*}}foo{{.*}}!type !{{[0-9]+}}
    // CHECK:       start:
    // CHECK:       [[TT:%.+]] = call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%0}}, metadata !"{{[[:print:]]+}}")
    // CHECK-NEXT:  br i1 [[TT]], label %type_test.pass, label %type_test.fail
    // CHECK:       type_test.pass:
    // CHECK-NEXT:  {{%.+}} = call noundef i32 %f(i32 noundef %arg)
    // CHECK-NEXT:  br label %bb1
    // CHECK:       type_test.fail:
    // CHECK-NEXT:  call void @llvm.trap()
    // CHECK-NEXT:  unreachable
    f(arg)
}
