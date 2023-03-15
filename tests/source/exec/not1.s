	.text
	.globl main
main:
	movq $65, %rdi
	movq $0, %r9
	cmpq %r9, $0
	sete %r9b
	addq %r9, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %rcx
	cmpq %rcx, $0
	sete %cl
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

