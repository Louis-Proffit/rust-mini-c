	.text
	.globl main
many:
	pushq %rbp
	movq %rsp, %rbp
	addq $-112, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq %rcx, -32(%rbp)
	movq %r8, -40(%rbp)
	movq %r9, -48(%rbp)
	movq 16(%rbp), %r10
	movq %r10, -56(%rbp)
	movq 24(%rbp), %r10
	movq %r10, -64(%rbp)
	movq 32(%rbp), %r10
	movq %r10, -72(%rbp)
	movq 40(%rbp), %r10
	movq %r10, -80(%rbp)
	movq $64, %rdi
	movq -8(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	movq $64, %rdi
	movq -16(%rbp), %rdx
	addq %rdx, %rdi
	call putchar
	movq $64, %rdi
	movq -24(%rbp), %rax
	addq %rax, %rdi
	call putchar
	movq $64, %rdi
	movq -32(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	movq $64, %rdi
	movq -40(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	movq $64, %rdi
	movq -48(%rbp), %r9
	addq %r9, %rdi
	call putchar
	movq $64, %rdi
	movq -56(%rbp), %rcx
	addq %rcx, %rdi
	call putchar
	movq $64, %rdi
	movq -64(%rbp), %rax
	addq %rax, %rdi
	call putchar
	movq $64, %rdi
	movq -72(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	movq $64, %rdi
	movq -80(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq -8(%rbp), %rax
	movq $10, -112(%rbp)
	cmpq -112(%rbp), %rax
	setl %al
	testq %rax, %rax
	jnz L3816
L3805:
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L3816:
	movq -16(%rbp), %rdi
	movq -24(%rbp), %rsi
	movq -32(%rbp), %rdx
	movq -40(%rbp), %rcx
	movq -48(%rbp), %r8
	movq -56(%rbp), %r9
	movq -64(%rbp), %r10
	movq %r10, -104(%rbp)
	movq -72(%rbp), %r10
	movq %r10, -96(%rbp)
	movq -80(%rbp), %r10
	movq %r10, -88(%rbp)
	movq -8(%rbp), %rax
	pushq %rax
	pushq -88(%rbp)
	pushq -96(%rbp)
	pushq -104(%rbp)
	call many
	addq $32, %rsp
	jmp L3805
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-24, %rsp
	movq $1, %rdi
	movq $2, %rsi
	movq $3, %rdx
	movq $4, %rcx
	movq $5, %r8
	movq $6, %r9
	movq $7, -24(%rbp)
	movq $8, -16(%rbp)
	movq $9, -8(%rbp)
	movq $10, %rax
	pushq %rax
	pushq -8(%rbp)
	pushq -16(%rbp)
	pushq -24(%rbp)
	call many
	addq $32, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

