	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $65, %rdi
	movq $0, %rsi
	call f
	movq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rsi
	call f
	movq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $2, %rsi
	call f
	movq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $3, %rsi
	call f
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdx
	movq -8(%rbp), %rdi
	call putchar
	movq $1, %rcx
	addq %rcx, -8(%rbp)
	movq -8(%rbp), %rdx
	movq -8(%rbp), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
f:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, %rax
	movq %rsi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	addq -16(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

