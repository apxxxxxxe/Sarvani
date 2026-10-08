$isRequirementsInstalled = $true

function Send-SSTP {
  param(
    [string]$message,
    [string]$uniqueid
  )

  $tcpClient = New-Object System.Net.Sockets.TcpClient("localhost", 9801)
  try {
    $stream = $tcpClient.GetStream()
    $writer = New-Object System.IO.StreamWriter($stream, [System.Text.UTF8Encoding]::new($false))
    $writer.WriteLine("SEND SSTP/1.0")
    $writer.WriteLine("Charset: UTF-8")
    $writer.WriteLine("Sender: Sarvani Builder")
    $writer.WriteLine("Script: $message")
    $writer.WriteLine("Option: notranslate")
    if ($uniqueid) {
      $writer.WriteLine("ID: $uniqueid")
    }
    $writer.WriteLine()
    $writer.Flush()

    # 応答を読み切ることでスクリプト実行完了まで待つ
    $reader = New-Object System.IO.StreamReader($stream, [System.Text.Encoding]::UTF8)
    $response = $reader.ReadLine()
    Write-Host "SSTP: $response"
  } finally {
    $tcpClient.Close()
  }
}

# check if magick is installed
if (!(Get-Command "magick" -ErrorAction SilentlyContinue)) {
    Write-Host "magick is not installed"
    $isRequirementsInstalled = $false
}

# check if cargo is installed
if (!(Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "cargo is not installed"
    $isRequirementsInstalled = $false
}

if (!$isRequirementsInstalled) {
    Write-Host "Requirements are not installed. Please install the requirements and try again."
    exit 1
}

# ./ghost/master/debug が存在するか確認し、存在するなら内容を読み込む
if (Test-Path $PSScriptRoot\ghost\master\debug) {
  $uniqueid = Get-Content $PSScriptRoot\ghost\master\debug
}

Send-SSTP "\1\_qビルド中\![unload,shiori]\e" $uniqueid

Start-Sleep -Seconds 1

cd $PSScriptRoot\ghost\master
cargo build --release

# unload完了前だとDLLが使用中のことがあるためリトライする
$copied = $false
for ($retry = 0; $retry -lt 20; $retry++) {
  try {
    Copy-Item -Force -ErrorAction Stop $PSScriptRoot\ghost\master\target\i686-pc-windows-msvc\release\sarvani.dll $PSScriptRoot\ghost\master\
    $copied = $true
    break
  } catch {
    Write-Host "sarvani.dll is in use, retrying... ($($retry + 1)/20)"
    Start-Sleep -Milliseconds 500
  }
}
if (!$copied) {
  Write-Host "Failed to copy sarvani.dll: file is still in use."
  Send-SSTP "\1\_qビルド失敗\![reload,ghost]\e" $uniqueid
  exit 1
}

Send-SSTP "\1\_qビルド完了\![reload,ghost]\e" $uniqueid
