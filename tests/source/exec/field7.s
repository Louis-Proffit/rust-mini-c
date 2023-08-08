	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -16(%rbp)
	movq -16(%rbp), %r9
	movq -16(%rbp), %r10
	movq %r10, -8(%rbp)
	movq -8(%rbp), %r8
	movq $65, %rax
	movq -8(%rbp), %r8
	movq %rax, 0(%r8)
	movq -16(%rbp), %rax
	movq 0(%rax), %rdi
	call putchar
	movq -8(%rbp), %r9
	movq 0(%r9), %rdi
	call putchar
	movq $66, %rsi
	movq -8(%rbp), %rdx
	movq %rsi, 8(%rdx)
	movq -16(%rbp), %rdi
	movq 8(%rdi), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 8(%rax), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

