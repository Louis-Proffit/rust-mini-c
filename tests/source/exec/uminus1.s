	.text
	.globl main
main:
	movq $66, %rdi
	movq $1, %r9
	movq $0, %rax
	subq %r9, %rax
	addq %rax, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %rsi
	movq $0, %rax
	subq %rsi, %rax
	movq $0, %r8
	subq %rax, %r8
	addq %r8, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

