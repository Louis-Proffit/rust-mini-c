	.text
	.globl main
main:
	movq $65, %rdi
	movq $1, %rcx
	testq %rcx, %rcx
	jnz L4087
	movq $1, %rcx
	testq %rcx, %rcx
	jz L4086
L4087:
	movq $1, %rcx
L4086:
	addq %rcx, %rdi
	call putchar
	movq $65, %rdi
	movq $0, %rdx
	testq %rdx, %rdx
	jnz L4079
	movq $2, %rdx
	testq %rdx, %rdx
	jz L4078
L4079:
	movq $1, %rdx
L4078:
	addq %rdx, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rcx
	testq %rcx, %rcx
	jnz L4071
	movq $0, %rcx
	testq %rcx, %rcx
	jz L4070
L4071:
	movq $1, %rcx
L4070:
	addq %rcx, %rdi
	call putchar
	movq $65, %rdi
	movq $0, %rsi
	testq %rsi, %rsi
	jnz L4063
	movq $0, %rsi
	testq %rsi, %rsi
	jz L4062
L4063:
	movq $1, %rsi
L4062:
	addq %rsi, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	jmp L4062
	jmp L4063
	jmp L4070
	jmp L4071
	jmp L4078
	jmp L4079
	jmp L4086
	jmp L4087
	.data

