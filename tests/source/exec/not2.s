	.text
	.globl main
main:
	movq $0, %rax
	movq %rax, %rcx
	movq $65, %rdi
	cmpq %rax, $0
	sete %al
	addq %rax, %rdi
	call putchar
	addq $0, %rsp
	movq $1, %rax
	movq %rax, %r8
	movq $65, %rdi
	cmpq %rax, $0
	sete %al
	addq %rax, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

