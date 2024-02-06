	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rsi
	movq $24, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rax
	movq $16, %rdi
	call malloc
	movq -8(%rbp), %rdi
	movq %rax, 8(%rdi)
	movq $65, %rdx
	movq -8(%rbp), %rdi
	movq %rdx, 0(%rdi)
	movq $66, %rax
	movq -8(%rbp), %rdi
	movq %rax, 16(%rdi)
	movq $120, %r9
	movq -8(%rbp), %r8
	movq 8(%r8), %rdi
	movq %r9, 0(%rdi)
	movq $121, %rcx
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rdx
	movq %rcx, 8(%rdx)
	movq -8(%rbp), %rdx
	movq 0(%rdx), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 8(%rax), %rdx
	movq 0(%rdx), %rdi
	call putchar
	movq -8(%rbp), %rcx
	movq 8(%rcx), %rdi
	movq 8(%rdi), %rdi
	call putchar
	movq -8(%rbp), %rcx
	movq 16(%rcx), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $88, %r9
	movq -16(%rbp), %rdi
	movq %r9, 0(%rdi)
	movq $89, %rcx
	movq -16(%rbp), %rax
	movq %rcx, 8(%rax)
	movq -8(%rbp), %r8
	movq 0(%r8), %rdi
	call putchar
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rax
	movq 0(%rax), %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rax
	movq 8(%rax), %rdi
	call putchar
	movq -8(%rbp), %rcx
	movq 16(%rcx), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq -16(%rbp), %r9
	movq -8(%rbp), %rsi
	movq %r9, 8(%rsi)
	movq -8(%rbp), %rsi
	movq 0(%rsi), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 8(%rax), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq -8(%rbp), %rcx
	movq 8(%rcx), %r8
	movq 8(%r8), %rdi
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

