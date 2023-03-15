	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rcx
L4874:
	movq -8(%rbp), %rsi
	testq %rsi, %rsi
	jz L4863
	movq $65, %rdi
	movq -8(%rbp), %rax
	addq %rax, %rdi
	call putchar
	addq $0, %rsp
	movq $1, %rax
	subq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	jmp L4874
L4863:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

