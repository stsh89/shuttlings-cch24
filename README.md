# Christmas Code Hunt 2024

[Shuttle's Christmas Code Hunt](https://www.shuttle.dev/cch)

## Day 9: The Cookies and Milk Factory

Test script:

```powershell
$jobs = 1..20 | ForEach-Object {Start-Job -ScriptBlock {curl --silent -X POST http://localhost:8000/9/milk}}
$jobs | ForEach-Object { $_ | Wait-Job; Receive-Job -Job $_}
$jobs | Remove-Job
```
