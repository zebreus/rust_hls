; ModuleID = '<stdin>'
source_filename = "llvm-link"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none)
define noundef i32 @other_function(i32 noundef %a, i32 noundef %b) unnamed_addr #0 {
start:
  %a1 = add i32 %b, -1
  %_0 = mul i32 %a1, %a
  ret i32 %_0
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) "probe-stack"="inline-asm" "target-cpu"="generic" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
