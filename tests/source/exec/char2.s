	.text
	.globl main
main:
	movq $65, %rdi
	movq %rdi, %r8
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

