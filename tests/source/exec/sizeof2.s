	.text
	.globl main
main:
	movq $65, %rdi
	movq $8, %rsi
	addq %rsi, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $16, %r9
	addq %r9, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

