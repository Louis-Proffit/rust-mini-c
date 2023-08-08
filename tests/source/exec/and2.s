	.text
	.globl main
main:
	movq $65, %rdi
	movq $0, %rax
	testq %rax, %rax
	jz L114
	movq $1, %rax
	testq %rax, %rax
	jz L114
	movq $1, %rax
L114:
	addq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $0, %rsi
	testq %rsi, %rsi
	jz L100
	movq $2, %rsi
	testq %rsi, %rsi
	jz L100
	movq $1, %rsi
L100:
	addq %rsi, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rsi
	testq %rsi, %rsi
	jz L82
	movq $0, %rsi
	testq %rsi, %rsi
	jz L82
	movq $1, %rsi
L82:
	addq %rsi, %rdi
	call putchar
	movq $65, %rdi
	movq $0, %r8
	testq %r8, %r8
	jz L64
	movq $0, %r8
	testq %r8, %r8
	jz L64
	movq $1, %r8
L64:
	addq %r8, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	jmp L64
	jmp L64
	jmp L82
	jmp L82
	jmp L100
	jmp L100
	jmp L114
	jmp L114
	.data

