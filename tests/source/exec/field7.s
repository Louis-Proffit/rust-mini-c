	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -16(%rbp)
	addq $0, %rsp
	movq -16(%rbp), %rsi
	movq -16(%rbp), %r10
	movq %r10, -8(%rbp)
	movq -8(%rbp), %rsi
	movq $65, %r9
	movq -8(%rbp), %r8
	movq %r8, 0(%r9)
	movq -16(%rbp), %rax
	movq 0(%rax), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rdi
	movq 0(%rdi), %rdi
	call putchar
	addq $0, %rsp
	movq $66, %r8
	movq -8(%rbp), %rcx
	movq %rcx, 8(%r8)
	movq -16(%rbp), %rdi
	movq 8(%rdi), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rax
	movq 8(%rax), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

