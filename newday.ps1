param($daynum)
$daynum = $PSBoundParameters["daynum"]

$null=New-Item -Force src/day$daynum.rs -Value (Get-Content -Raw src/template.rs)
$null=New-Item -Force src/inputs/day$daynum.txt -Value $null
$null=New-Item -Force src/inputs/day$daynum-test.txt -Value $null
echo "Add line to main.rs: mod day$daynum;"
echo "Add line to main.rs: [day$daynum::pt1, day$daynum::pt2],"