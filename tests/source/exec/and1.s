	.text
	.globl main
main:
	movq $65, %rdi
	movq $1, %rsi
	testq %rsi, %rsi
	jz L56
	movq $1, %rsi
	testq %rsi, %rsi
	jz L56
	movq $1, %rsi
L56:
	addq %rsi, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rcx
	testq %rcx, %rcx
	jz L33
	movq $2, %rcx
	testq %rcx, %rcx
	jz L33
	movq $1, %rcx
L33:
	addq %rcx, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %r9
	testq %r9, %r9
	jz L15
	movq $0, %r9
	testq %r9, %r9
	jz L15
	movq $1, %r9
L15:
	addq %r9, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	jmp L15
	jmp L15
	jmp L33
	jmp L33
	jmp L56
	jmp L56
	.data

