---
title: "WhisperKitの使い方と評価 音声処理AIをMacとiOSで動かす"
emoji: "🎤"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: [whisper, whisperkit, ai, swift, oss]
published: true
---

Chat GPTを革切りに、AI関連のキャッチアップをしている[Ryo24](https://twitter.com/r0227n_)です。
今回は[whisper.cpp](https://github.com/ggerganov/whisper.cpp)をApple製品上で動かすのに最適化したOSS、[WhisperKit](https://github.com/argmaxinc/WhisperKit?tab=readme-ov-file)について軽く勉強したので、使い方及び感想をまとめました。

# WhisperKitとは

簡単に説明すると

- Apple製品に最適化された音声処理AI
- OSS
- Macで動かす前提だが、`whisper.cpp`より精度が高い

という特徴があります。
↓の記事に詳細な説明が書かれているため、興味がある方は参考にしてみてください。
https://buildersbox.corp-sansan.com/entry/2024/04/24/110000

# 使い方

`Swift Package Manager`でプロジェクトに導入可能ですし、Swift CLIで使うこともできます。
本記事では、Swift CLIの使い方を紹介します。

## Swift CLIでの使い方

### 環境構築

ステップは以下の通りです。

1. git clone https://github.com/argmaxinc/whisperkit.git
2. cd whisperkit
3. make setup
4. make download-model MODEL=large-v3
5. cd Models/whisperkit-coreml
6. git lfs pull

以上6ステップで、WhisperKitを使う準備が整います。

#### README.mdに書かれていない注意点

README.mdには書かれていないが、`git lfs pull`を実行する必要があります。

「`make download-model`コマンドでLLMをダウンロードしてもGit LFS linksのみダウンロードされ、実際のモデルファイルはダウンロードされない」という問題があるため、README.mdに書かれている手順に従ったが、**モデルファイルがダウンロードされない**ためエラーが発生します。この問題を解決するために`git lfs pull`を実行する必要があります。
(2024/05/05時点)
https://github.com/argmaxinc/WhisperKit/issues/4

:::details 確認したエラー

```bash
Error: The file “config.json” couldn’t be opened.

----

Fetching https://github.com/huggingface/swift-transformers.git
Fetching https://github.com/apple/swift-argument-parser.git
Fetched https://github.com/apple/swift-argument-parser.git from cache (3.83s)
Fetched https://github.com/huggingface/swift-transformers.git from cache (3.83s)
Computing version for https://github.com/apple/swift-argument-parser.git
Computed https://github.com/apple/swift-argument-parser.git at 1.3.0 (0.24s)
Computing version for https://github.com/huggingface/swift-transformers.git
Computed https://github.com/huggingface/swift-transformers.git at 0.1.7 (0.21s)
Computed https://github.com/apple/swift-argument-parser.git at 1.3.0 (0.00s)
Computed https://github.com/huggingface/swift-transformers.git at 0.1.7 (0.00s)
Creating working copy for https://github.com/huggingface/swift-transformers.git
Working copy of https://github.com/huggingface/swift-transformers.git resolved at 0.1.7
Creating working copy for https://github.com/apple/swift-argument-parser.git
Working copy of https://github.com/apple/swift-argument-parser.git resolved at 1.3.0
warning: 'whisperkit': found 1 file(s) which are unhandled; explicitly declare them as resources or exclude from the target
    /Users/<username>/dev/whisperkit/sample.mp3
Building for debugging...
[112/112] Applying whisperkit-cli
Build complete! (11.02s)
Error: Unable to load model: file:///Users/<username>/dev/whisperkit/Models/whisperkit-coreml/openai_whisper-large-v3/MelSpectrogram.mlmodelc/. Compile the model with Xcode or `MLModel.compileModel(at:)`.
```

:::

### 実行

- 音声ファイル(`wav,mp3,m4a,flac`)の読み込み、テキストに書き出す: `--audio-path`オプション
  - `swift run whisperkit-cli transcribe --model-path "Models/whisperkit-coreml/openai_whisper-large-v3" --audio-path ./sample.mp3`
- Stream形式で音声を読み込み、テキストに書き出す: `--stream`オプション
  - `swift run whisperkit-cli transcribe --model-path "Models/whisperkit-coreml/openai_whisper-large-v3" --stream`

の2パターンで音声をテキストに変換することができます。

:::details transcribeコマンドのオプション一覧

```bash
swift run whisperkit-cli transcribe --he
---
OVERVIEW: Transcribe audio to text using WhisperKit

USAGE: whisperkit-cli transcribe [<options>] [<supress-tokens> ...]

ARGUMENTS:
  <supress-tokens>        Supress given tokens in the output

OPTIONS:
  --audio-path <audio-path>
                          Paths to audio files
  --audio-folder <audio-folder>
                          Path to a folder containing audio files
  --model-path <model-path>
                          Path of model files
  --model <model>         Model to download if no modelPath is provided
  --model-prefix <model-prefix>
                          Text to add in front of the model name to specify
                          between different types of the same variant (values:
                          "openai", "distil") (default: openai)
  --download-model-path <download-model-path>
                          Path to save the downloaded model
  --download-tokenizer-path <download-tokenizer-path>
                          Path to save the downloaded tokenizer files
  --audio-encoder-compute-units <audio-encoder-compute-units>
                          Compute units for audio encoder model with
                          {all,cpuOnly,cpuAndGPU,cpuAndNeuralEngine,random}
                          (values: all, cpuAndGPU, cpuOnly, cpuAndNeuralEngine,
                          random; default: cpuAndNeuralEngine)
  --text-decoder-compute-units <text-decoder-compute-units>
                          Compute units for text decoder model with
                          {all,cpuOnly,cpuAndGPU,cpuAndNeuralEngine,random}
                          (values: all, cpuAndGPU, cpuOnly, cpuAndNeuralEngine,
                          random; default: cpuAndNeuralEngine)
  --verbose               Verbose mode
  --task <task>           Task to perform (transcribe or translate) (default:
                          transcribe)
  --language <language>   Language spoken in the audio
  --temperature <temperature>
                          Temperature to use for sampling (default: 0.0)
  --temperature-increment-on-fallback <temperature-increment-on-fallback>
                          Temperature to increase on fallbacks during decoding
                          (default: 0.2)
  --temperature-fallback-count <temperature-fallback-count>
                          Number of times to increase temperature when falling
                          back during decoding (default: 5)
  --best-of <best-of>     Number of candidates when sampling with non-zero
                          temperature (default: 5)
  --use-prefill-prompt    Force initial prompt tokens based on language, task,
                          and timestamp options
  --use-prefill-cache     Use decoder prefill data for faster initial decoding
  --skip-special-tokens   Skip special tokens in the output
  --without-timestamps    Force no timestamps when decoding
  --word-timestamps       Add timestamps for each word in the output
  --prefix <prefix>       Force prefix text when decoding
  --prompt <prompt>       Condition on this text when decoding
  --compression-ratio-threshold <compression-ratio-threshold>
                          Gzip compression ratio threshold for decoding failure
  --logprob-threshold <logprob-threshold>
                          Average log probability threshold for decoding failure
  --first-token-log-prob-threshold <first-token-log-prob-threshold>
                          Log probability threshold for first token decoding
                          failure
  --no-speech-threshold <no-speech-threshold>
                          Probability threshold to consider a segment as silence
  --report                Output a report of the results
  --report-path <report-path>
                          Directory to save the report (default: .)
  --stream                Process audio directly from the microphone
  --stream-simulated      Simulate streaming transcription using the input
                          audio file
  --concurrent-worker-count <concurrent-worker-count>
                          Maximum concurrent inference, might be helpful when
                          processing more than 1 audio file at the same time. 0
                          means unlimited (default: 0)
  -h, --help              Show help information.
```

:::

# 実行結果

Swift CLI及びExampleを一通り試し、以下の結果及び感想を抱きました。
また動作が安定しておらず、同一環境で実行しても正常実行されないことがあるため、注意が必要です。

## 実行環境

- WhisperKit: v0.6.1
- PC: M2 MBA
- OS: macOS Sonoma 14.4.1
- iPhone: iPhone 15 Pro Max(エミュレータ)
- iOS: 17.4
- LLM: openai_whisper-large-v3

### Swift CLIでの実行結果

#### 音声ファイル(.mp3)を読み込み、テキストに変換

[CM原稿（せっけん）](https://pro-video.jp/voice/announce/)を使い、音声ファイルをテキストに変換しました。

```bash 変換結果
無添加のシャボン玉石鹸ならもう安心!天然の保湿成分が含まれるため、肌にうるおいを与え、健やかに保ちます。お肌のことでお悩みの方は、ぜひ一度、無添加シャボン玉石鹸をお試しください。お求めは、0120-0055-95まで。
```

漢字や句読点、数字など正確に変換されているため、非常に高い精度を持っていると感じました。

#### Stream形式で音声を読み込み、テキストに変換

```bash 変換結果
Unconfirmed segment: <|startoftranscript|><|nospeech|><|endoftext|>
Unconfirmed segment: <|startoftranscript|><|ja|><|transcribe|><|notimestamps|>マイクテースマイクテースこんにちは<|endoftext|>
Current text: 
---
Unconfirmed segment: <|startoftranscript|><|nospeech|><|endoftext|>
Unconfirmed segment: <|startoftranscript|><|ja|><|transcribe|><|notimestamps|>マイクテースマイクテースこんにちは<|endoftext|>
Current text: Waiting for speech...
```

発生から文字起こしまでかなりタイムラグがありました。また話す時に一定間隔で間を設けましたが、文節を意識した文字起こしはされず、音声をそのまま文字に変換しているだけのように感じます。
ですが[whisper.cppのexample/stream](https://github.com/ggerganov/whisper.cpp/tree/master/examples/stream)と比較すると、精度が高く、最適化されていると感じました。

### Exampleでの実行結果

#### 音声ファイル(.mp3)を読み込み、テキストに変換

https://twitter.com/i/status/1786251828134600895

![](https://storage.googleapis.com/zenn-user-upload/98d186c6bf16-20240505.png)
(横軸が音声ファイルの言語 / 縦軸が言語設定)

基本的に日本語の音声ファイルは変換不可だが、英語の音声ファイルは変換可能でした。
また、言語設定の影響はあまりないように感じました。

#### Stream形式で音声を読み込み、テキストに変換

日本語英語の発生&言語設定どちらも変換できませんでした。
(マイクや発音など環境が悪い可能性があるので、あくまで本結果は参考程度にお願いします。)

# 感想

まず最初に、WhisperKitは正式リリースされておらず、まだ開発途中であることを理解しておく必要があります。また本家大元のWhisperに関しても、exampleでモバイル上での動作を書いていますが、メインの実行環境はPCであることを理解した上で、個人的な感想を述べます。

**Mac向けのアプリケーションでの利用は有りだが、iOS向けのアプリケーションでの利用は難しい**

っというのが、今回の感想です。
Swift CLIでの実行結果を見る限り、音声ファイルの読み込み及びStreamのクオリティは高く、特定のアプリに実装されていたら、好印象を持てます。しかし、iOSは精度云々の問題ではなく、**動く or 動かない**のレベルであり、実用的なアプリケーションに組み込むのは難しいです。
これは純粋に計算リソースの違いであり、PCとモバイルの性能差が大きいため、現段階ではモバイル上でWhisperを動かすのは現実的ではないと思います。
加えて、内部でSwiftからPythonを動かしている部分の動作が不安定であり、同一環境で実行しても正常実行されないことが多いため、品質の担保が難しいです。

あくまで正式リリースされていないため、今後のアップデートで改善される可能性もあり、今後の動向に注目していきたいと思います。

https://zenn.dev/r0227n/scraps/d9e558f3f30ff9
