	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-72, %rsp
	movq $1, %rdi
	movq $2, %rsi
	movq $3, %rdx
	movq $4, %rcx
	movq $5, %r8
	movq $6, %r9
	movq $7, -72(%rbp)
	movq $8, %rax
	pushq %rax
	pushq -72(%rbp)
	call f
	addq $16, %rsp
	movq $1, %rdi
	movq $1, %rsi
	movq $1, %rdx
	movq $1, %rcx
	movq $1, %r8
	movq $1, %r9
	movq $1, -64(%rbp)
	movq $1, %rax
	pushq %rax
	pushq -64(%rbp)
	call f
	addq $16, %rsp
	movq $2, %rdi
	movq $2, %rsi
	movq $2, %rdx
	movq $2, %rcx
	movq $2, %r8
	movq $2, %r9
	movq $2, -56(%rbp)
	movq $2, %rax
	pushq %rax
	pushq -56(%rbp)
	call f
	addq $16, %rsp
	movq $8, %rdi
	movq $1, %rsi
	movq $2, %rdx
	movq $3, %rcx
	movq $4, %r8
	movq $5, %r9
	movq $6, -48(%rbp)
	movq $7, %rax
	pushq %rax
	pushq -48(%rbp)
	call f
	addq $16, %rsp
	movq $7, %rdi
	movq $6, %rsi
	movq $5, %rdx
	movq $4, %rcx
	movq $3, %r8
	movq $2, %r9
	movq $1, -40(%rbp)
	movq $8, %rax
	pushq %rax
	pushq -40(%rbp)
	call f
	addq $16, %rsp
	movq $1, %rdx
	movq $0, %rdi
	subq %rdx, %rdi
	movq $2, %rax
	movq $0, %rsi
	subq %rax, %rsi
	movq $3, %r9
	movq $0, %rdx
	subq %r9, %rdx
	movq $4, %r9
	movq $0, %rcx
	subq %r9, %rcx
	movq $5, %r9
	movq $0, %r8
	subq %r9, %r8
	movq $6, %rax
	movq $0, %r9
	subq %rax, %r9
	movq $7, %rax
	movq $0, -32(%rbp)
	subq %rax, -32(%rbp)
	movq $8, -24(%rbp)
	movq $0, %rax
	subq -24(%rbp), %rax
	pushq %rax
	pushq -32(%rbp)
	call f
	addq $16, %rsp
	movq $1, %rdi
	movq $2, %rax
	movq $0, %rsi
	subq %rax, %rsi
	movq $3, %rdx
	movq $4, %rax
	movq $0, %rcx
	subq %rax, %rcx
	movq $5, %r8
	movq $6, %rax
	movq $0, %r9
	subq %rax, %r9
	movq $7, -16(%rbp)
	movq $8, -8(%rbp)
	movq $0, %rax
	subq -8(%rbp), %rax
	pushq %rax
	pushq -16(%rbp)
	call f
	addq $16, %rsp
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
f:
	pushq %rbp
	movq %rsp, %rbp
	addq $-120, %rsp
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
	movq $9, -96(%rbp)
	movq -96(%rbp), %rax
	movq $10, -104(%rbp)
	movq -104(%rbp), %rsi
	movq $11, -112(%rbp)
	movq -112(%rbp), %rdi
	movq $12, -120(%rbp)
	movq -120(%rbp), %rcx
	movq $13, -80(%rbp)
	movq -80(%rbp), %rax
	movq $14, -88(%rbp)
	movq -88(%rbp), %rdi
	movq $15, -72(%rbp)
	movq -72(%rbp), %r8
	movq $65, %rdi
	movq -8(%rbp), %rax
	movq -16(%rbp), %r9
	imulq %r9, %rax
	addq %rax, %rdi
	movq -24(%rbp), %rax
	movq -32(%rbp), %rcx
	cqto
	idivq %rcx
	movq -40(%rbp), %r8
	cqto
	idivq %r8
	movq -48(%rbp), %r8
	cqto
	idivq %r8
	subq %rax, %rdi
	movq -56(%rbp), %rdx
	addq %rdx, %rdi
	movq -64(%rbp), %rsi
	subq %rsi, %rdi
	movq -96(%rbp), %r9
	movq -104(%rbp), %rax
	cmpq %rax, %r9
	sete %r9b
	addq %r9, %rdi
	movq -112(%rbp), %rax
	movq -120(%rbp), %r8
	cmpq %r8, %rax
	setl %al
	subq %rax, %rdi
	movq -80(%rbp), %rdx
	movq -88(%rbp), %rcx
	cmpq %rcx, %rdx
	setle %dl
	subq %rdx, %rdi
	movq -8(%rbp), %r8
	movq -16(%rbp), %rsi
	cmpq %rsi, %r8
	setg %r8b
	movq -24(%rbp), %rax
	cmpq %rax, %r8
	setg %r8b
	addq %r8, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rcx
	movq $0, %rax
	subq %rcx, %rax
	movq -64(%rbp), %r8
	imulq %r8, %rax
	addq %rax, %rdi
	movq -8(%rbp), %rax
	movq -16(%rbp), %r8
	imulq %r8, %rax
	movq -24(%rbp), %rcx
	movq -72(%rbp), %rsi
	cmpq %rsi, %rcx
	setl %cl
	imulq %rcx, %rax
	addq %rax, %rdi
	movq -88(%rbp), %rax
	movq -80(%rbp), %r9
	movq -96(%rbp), %r8
	addq %r8, %r9
	cmpq %r9, %rax
	setg %al
	movq -16(%rbp), %rsi
	imulq %rsi, %rax
	movq -16(%rbp), %rcx
	cqto
	idivq %rcx
	movq -16(%rbp), %rsi
	imulq %rsi, %rax
	movq -16(%rbp), %r9
	cqto
	idivq %r9
	subq %rax, %rdi
	movq -112(%rbp), %r9
	movq $0, %rsi
	subq %r9, %rsi
	addq %rsi, %rdi
	movq -112(%rbp), %rdx
	movq -112(%rbp), %rax
	cmpq %rax, %rdx
	setne %dl
	movq $0, %rcx
	subq %rdx, %rcx
	addq %rcx, %rdi
	movq -32(%rbp), %r8
	addq %r8, %rdi
	movq -56(%rbp), %rdx
	addq %rdx, %rdi
	call putchar
	movq $65, %rdi
	movq -8(%rbp), %rax
	movq -16(%rbp), %r9
	imulq %r9, %rax
	movq -8(%rbp), %rcx
	movq -16(%rbp), %r9
	movq $2, %r8
	cmpq %r8, %r9
	setne %r9b
	movq $4, %rsi
	imulq %rsi, %r9
	cmpq %r9, %rcx
	setl %cl
	imulq %rcx, %rax
	movq -24(%rbp), %r9
	cqto
	idivq %r9
	movq -32(%rbp), %r8
	imulq %r8, %rax
	movq -40(%rbp), %rcx
	cqto
	idivq %rcx
	movq -48(%rbp), %r9
	cqto
	idivq %r9
	addq %rax, %rdi
	movq -56(%rbp), %rax
	movq -64(%rbp), %r8
	cqto
	idivq %r8
	movq -96(%rbp), %r8
	imulq %r8, %rax
	addq %rax, %rdi
	movq -104(%rbp), %rax
	movq -112(%rbp), %r8
	imulq %r8, %rax
	movq -120(%rbp), %rcx
	cqto
	idivq %rcx
	addq %rax, %rdi
	movq $1, %rax
	movq -80(%rbp), %rsi
	cqto
	idivq %rsi
	addq %rax, %rdi
	movq $30, %rax
	movq -88(%rbp), %r9
	cqto
	idivq %r9
	addq %rax, %rdi
	movq -56(%rbp), %rax
	movq -88(%rbp), %rsi
	movq -88(%rbp), %rcx
	cmpq %rcx, %rsi
	sete %sil
	cqto
	idivq %rsi
	addq %rax, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

