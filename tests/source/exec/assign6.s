	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rcx
	movq $24, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rsi
	movq $16, %rdi
	call malloc
	movq -8(%rbp), %rcx
	movq %rax, 8(%rcx)
	movq $65, %rax
	movq -8(%rbp), %rsi
	movq %rax, 0(%rsi)
	movq $66, %rdi
	movq -8(%rbp), %rax
	movq %rdi, 16(%rax)
	movq $120, %r8
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rsi
	movq %r8, 0(%rsi)
	movq $121, %r9
	movq -8(%rbp), %rcx
	movq 8(%rcx), %rdi
	movq %r9, 8(%rdi)
	movq -8(%rbp), %rdi
	movq 0(%rdi), %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rdx
	movq 0(%rdx), %rdi
	call putchar
	movq -8(%rbp), %r9
	movq 8(%r9), %r8
	movq 8(%r8), %rdi
	call putchar
	movq -8(%rbp), %r8
	movq 16(%r8), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $88, %rax
	movq -16(%rbp), %rcx
	movq %rax, 0(%rcx)
	movq $89, %rax
	movq -16(%rbp), %r9
	movq %rax, 8(%r9)
	movq -8(%rbp), %rsi
	movq 0(%rsi), %rdi
	call putchar
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq -8(%rbp), %rcx
	movq 8(%rcx), %rcx
	movq 8(%rcx), %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq 16(%rdi), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq -16(%rbp), %r8
	movq -8(%rbp), %rdx
	movq %r8, 8(%rdx)
	movq -8(%rbp), %rdx
	movq 0(%rdx), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 8(%rax), %rdi
	movq 0(%rdi), %rdi
	call putchar
	movq -8(%rbp), %r9
	movq 8(%r9), %rdi
	movq 8(%rdi), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 16(%rax), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

