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
	movq -8(%rbp), %rax
	movq $16, %rdi
	call malloc
	movq -8(%rbp), %rdx
	movq %rax, 8(%rdx)
	movq $65, %r9
	movq -8(%rbp), %rsi
	movq %r9, 0(%rsi)
	movq $66, %rsi
	movq -8(%rbp), %rax
	movq %rsi, 16(%rax)
	movq $120, %rax
	movq -8(%rbp), %rdi
	movq 8(%rdi), %r8
	movq %rax, 0(%r8)
	movq $121, %r8
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rdi
	movq %r8, 8(%rdi)
	movq -8(%rbp), %rax
	movq 0(%rax), %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rax
	movq 0(%rax), %rdi
	call putchar
	movq -8(%rbp), %rcx
	movq 8(%rcx), %rdi
	movq 8(%rdi), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 16(%rax), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $88, %rsi
	movq -16(%rbp), %rdi
	movq %rsi, 0(%rdi)
	movq $89, %rdi
	movq -16(%rbp), %rcx
	movq %rdi, 8(%rcx)
	movq -8(%rbp), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq -8(%rbp), %rcx
	movq 8(%rcx), %r9
	movq 0(%r9), %rdi
	call putchar
	movq -8(%rbp), %r8
	movq 8(%r8), %rcx
	movq 8(%rcx), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 16(%rax), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq -16(%rbp), %rcx
	movq -8(%rbp), %rsi
	movq %rcx, 8(%rsi)
	movq -8(%rbp), %rax
	movq 0(%rax), %rdi
	call putchar
	movq -8(%rbp), %rax
	movq 8(%rax), %rdi
	movq 0(%rdi), %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rsi
	movq 8(%rsi), %rdi
	call putchar
	movq -8(%rbp), %r9
	movq 16(%r9), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

