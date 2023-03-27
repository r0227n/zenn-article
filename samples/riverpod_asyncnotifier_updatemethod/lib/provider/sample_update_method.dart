import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'sample_update_method.g.dart';

@riverpod
class SampleUpdateMethod extends _$SampleUpdateMethod {
  @override
  FutureOr<int> build() async {
    return Future.value(1);
  }

  // KO, "update" will likely never complete
  void caseA() {
    state = const AsyncLoading();
    update((data) => data = 2);
  }

  // OK
  void caseB() {
    update((data) {
      state = const AsyncLoading();
      return data += 1;
    });
  }

  // the parameter should be used. Otherwise use AsyncValue.guard
  void caseC() {
    update((unused) => unused += 1);
  }

  void sampleCaseC() {
    final oldState = state.asData?.value;
    state = const AsyncLoading();
    // AsyncValue で値を更新
    state = AsyncValue.data((oldState ?? -1) + 1);
  }
}
