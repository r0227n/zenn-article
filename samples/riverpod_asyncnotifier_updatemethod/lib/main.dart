import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'provider/sample_update_method.dart';

void main() {
  runApp(ProviderScope(child: const MyApp()));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(home: Home());
  }
}

class Home extends ConsumerWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final methods = ref.watch(sampleUpdateMethodProvider.notifier);

    return Scaffold(
      appBar: AppBar(title: const Text('Counter example')),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          Padding(
            padding: const EdgeInsets.only(bottom: 30),
            child: Consumer(
              builder: (context, ref, child) {
                final value = ref.watch(sampleUpdateMethodProvider);

                return value.when(
                  data: (count) => Text('$count'),
                  error: (err, stack) => Text('Error: $err'),
                  loading: () => Center(child: CircularProgressIndicator()),
                );
              },
            ),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            children: <ElevatedButton>[
              ElevatedButton(
                onPressed: () => methods.caseA(),
                child: const Text('Case A'),
              ),
              ElevatedButton(
                onPressed: () => methods.caseB(),
                child: const Text('Case B'),
              ),
              ElevatedButton(
                onPressed: () => methods.caseC(),
                child: const Text('Case C'),
              ),
              ElevatedButton(
                onPressed: () => methods.sampleCaseC(),
                child: const Text('Sample Case C'),
              ),
            ],
          )
        ],
      ),
    );
  }
}
