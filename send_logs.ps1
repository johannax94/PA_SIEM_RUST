$logs = Get-Content ".\account_compromise_logs.json" -Raw | ConvertFrom-Json

foreach ($log in $logs) {

    Invoke-RestMethod `
        -Uri "http://127.0.0.1:3000/logs" `
        -Method POST `
        -ContentType "application/json" `
        -Body ($log | ConvertTo-Json -Depth 10)

    Start-Sleep -Milliseconds 200
}