## sample.sp — HPVMx Scripting Engine Demo
## Run with: `script run internal/sample.sp World 5`

echo "=== HPVMx Script Demo ==="
echo "Greeting target: %%1"
echo "Loop limit: %%2"

set name = %%1
set limit = %%2
set counter = 0
set sum = 0

echo ""
echo "Counting down from %%2:"
while (counter < limit) {
  set counter = counter + 1
  set remaining = limit - counter
  if (remaining == 0) {
    echo "Done!"
  } elif (remaining < 3) {
    echo "Almost there... remaining: remaining"
  } else {
    echo "Step counter - remaining: remaining"
  }
  set sum = sum + counter
}

echo ""
echo "Summing 1 to 5 using for loop:"
set total = 0
for (set i = 1; i <= 5; i++) {
  set total = total + i
  echo "  i: i, running total: total"
}
echo "Final total: total"

echo ""
echo "Evaluating sum (sum = sum):"
if (sum > 100) {
  echo "Sum is large!"
} elif (sum > 10) {
  echo "Sum is medium."
} elif (sum > 0) {
  echo "Sum is small."
} else {
  echo "Sum is zero or negative."
}

echo ""
echo "Finding first even number > 0:"
for (set x = 1; x < 10; x++) {
  if (x % 2 != 0) {
    continue
  }
  if (x > 0) {
    echo "Found: x"
    break
  }
}

echo "Other Operations:"
echo ""

echo "ls:"
ls

echo "vmm:"
vmm info-adv

echo "net status:"
net status

echo "devices:"
devs


echo ""
echo "Script completed successfully!"
exit 0