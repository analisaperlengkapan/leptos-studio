with open('frontend/src/state/app_state.rs', 'r') as f:
    lines = f.readlines()

balance = 0
for i, line in enumerate(lines):
    for char in line:
        if char == '{':
            balance += 1
        elif char == '}':
            balance -= 1
    if balance < 0:
        print(f"Negative balance at line {i+1}: {line}")
        break

if balance != 0:
    print(f"Final balance: {balance}")
else:
    print("Braces are balanced.")
