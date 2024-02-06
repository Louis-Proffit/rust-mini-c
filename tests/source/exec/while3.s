	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rax
L4982:
	movq -8(%rbp), %rdi
	testq %rdi, %rdi
	jz L4971
	movq $65, %rdi
	movq -8(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	movq $1, %rax
	subq %rax, -8(%rbp)
	movq -8(%rbp), %r9
	jmp L4982
L4971:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

