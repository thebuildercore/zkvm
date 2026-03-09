; ModuleID = 'target.fc8032886893854c-cgu.0'
source_filename = "target.fc8032886893854c-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nonlazybind uwtable
define i32 @check_price(i32 %price) unnamed_addr #0 {
start:
  %result = alloca [4 x i8], align 4
  %_3 = icmp ugt i32 %price, 100
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  store i32 9, ptr %result, align 4
  br label %bb3

bb1:                                              ; preds = %start
  store i32 5, ptr %result, align 4
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  %_0 = load i32, ptr %result, align 4
  ret i32 %_0
}

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.93.0 (254b59607 2026-01-19)"}
