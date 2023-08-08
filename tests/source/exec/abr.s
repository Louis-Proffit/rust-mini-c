	.text
	.globl main
make:
	pushq %rbp
	movq %rsp, %rbp
	addq $-80, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq $24, %rdi
	call malloc
	movq %rax, -80(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -64(%rbp)
	movq %rax, -72(%rbp)
	movq -72(%rbp), %r10
	movq -64(%rbp), %r11
	movq %r11, 0(%r10)
	movq -16(%rbp), %r10
	movq %r10, -48(%rbp)
	movq %rax, -56(%rbp)
	movq -56(%rbp), %r10
	movq -48(%rbp), %r11
	movq %r11, 8(%r10)
	movq -24(%rbp), %r10
	movq %r10, -32(%rbp)
	movq %rax, -40(%rbp)
	movq -40(%rbp), %r10
	movq -32(%rbp), %r11
	movq %r11, 16(%r10)
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $1, %rdi
	movq $0, %rsi
	movq $0, %rdx
	call make
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rsi
	movq -8(%rbp), %rdi
	movq $17, %rsi
	call insere
	movq -8(%rbp), %rdi
	movq $5, %rsi
	call insere
	movq -8(%rbp), %rdi
	movq $8, %rsi
	call insere
	movq -8(%rbp), %rdi
	call print
	movq $10, %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq $5, %rsi
	call contient
	testq %rax, %rax
	jz L482
	movq -8(%rbp), %rdi
	movq $0, %rsi
	call contient
	movq $0, %r11
	cmpq %rax, %r11
	sete %al
	testq %rax, %rax
	jz L482
	movq $1, %rax
L482:
	testq %rax, %rax
	jz L476
	movq -8(%rbp), %rdi
	movq $17, %rsi
	call contient
	testq %rax, %rax
	jz L476
	movq $1, %rax
L476:
	testq %rax, %rax
	jz L469
	movq -8(%rbp), %rdi
	movq $3, %rsi
	call contient
	movq $0, %r11
	cmpq %rax, %r11
	sete %al
	testq %rax, %rax
	jz L469
	movq $1, %rax
L469:
	testq %rax, %rax
	jnz L468
L462:
	movq -8(%rbp), %rdi
	movq $42, %rsi
	call insere
	movq -8(%rbp), %rdi
	movq $1000, %rsi
	call insere
	movq -8(%rbp), %rdi
	movq $0, %rsi
	call insere
	movq -8(%rbp), %rdi
	call print
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L468:
	movq $111, %rdi
	call putchar
	movq $107, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	jmp L462
	jmp L469
	jmp L469
	jmp L476
	jmp L476
	jmp L482
	jmp L482
print_int:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq $10, %r8
	movq -16(%rbp), %rax
	cqto
	idivq %r8
	movq %rax, -16(%rbp)
	movq -16(%rbp), %r8
	movq -8(%rbp), %rax
	movq $9, %rdx
	cmpq %rdx, %rax
	setg %al
	testq %rax, %rax
	jnz L324
L322:
	movq $48, %rdi
	movq -8(%rbp), %r9
	movq $10, %rcx
	movq -16(%rbp), %rsi
	imulq %rsi, %rcx
	subq %rcx, %r9
	addq %r9, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L324:
	movq -16(%rbp), %rdi
	call print_int
	jmp L322
contient:
	pushq %rbp
	movq %rsp, %rbp
	addq $-96, %rsp
	movq %rdi, %rax
	movq %rsi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -80(%rbp)
	movq %rax, -96(%rbp)
	movq -96(%rbp), %r10
	movq 0(%r10), %r11
	movq %r11, -88(%rbp)
	movq -88(%rbp), %r10
	movq -80(%rbp), %r11
	cmpq %r10, %r11
	sete %r11b
	movq %r11, -80(%rbp)
	movq -80(%rbp), %r10
	testq %r10, %r10
	jnz L442
	movq -8(%rbp), %r10
	movq %r10, -40(%rbp)
	movq %rax, -72(%rbp)
	movq -72(%rbp), %r10
	movq 0(%r10), %r11
	movq %r11, -64(%rbp)
	movq -64(%rbp), %r10
	movq -40(%rbp), %r11
	cmpq %r10, %r11
	setl %r11b
	movq %r11, -40(%rbp)
	movq -40(%rbp), %r10
	testq %r10, %r10
	jz L430
	movq %rax, -56(%rbp)
	movq -56(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -40(%rbp)
	movq $0, -48(%rbp)
	movq -48(%rbp), %r10
	movq -40(%rbp), %r11
	cmpq %r10, %r11
	setne %r11b
	movq %r11, -40(%rbp)
	movq -40(%rbp), %r10
	testq %r10, %r10
	jz L430
	movq $1, -40(%rbp)
L430:
	movq -40(%rbp), %r10
	testq %r10, %r10
	jnz L429
	movq %rax, -32(%rbp)
	movq -32(%rbp), %r10
	movq 16(%r10), %r11
	movq %r11, -16(%rbp)
	movq $0, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r11
	cmpq %r10, %r11
	setne %r11b
	movq %r11, -16(%rbp)
	movq -16(%rbp), %r10
	testq %r10, %r10
	jnz L420
	movq $0, %rax
L415:
	movq %rbp, %rsp
	popq %rbp
	ret
L420:
	movq 16(%rax), %rdi
	movq -8(%rbp), %rsi
	call contient
	jmp L415
L429:
	movq 8(%rax), %rdi
	movq -8(%rbp), %rsi
	call contient
	jmp L415
	jmp L430
	jmp L430
L442:
	movq $1, %rax
	jmp L415
insere:
	pushq %rbp
	movq %rsp, %rbp
	addq $-48, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, %rax
	movq %rax, -32(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -48(%rbp)
	movq -48(%rbp), %r10
	movq 0(%r10), %r11
	movq %r11, -40(%rbp)
	movq -40(%rbp), %r10
	movq -32(%rbp), %r11
	cmpq %r10, %r11
	sete %r11b
	movq %r11, -32(%rbp)
	movq -32(%rbp), %r10
	testq %r10, %r10
	jnz L409
	movq %rax, %r8
	movq -8(%rbp), %rcx
	movq 0(%rcx), %rdi
	cmpq %rdi, %r8
	setl %r8b
	testq %r8, %r8
	jnz L403
	movq -8(%rbp), %rcx
	movq 16(%rcx), %r8
	movq $0, %r9
	cmpq %r9, %r8
	sete %r8b
	testq %r8, %r8
	jnz L383
	movq -8(%rbp), %rcx
	movq 16(%rcx), %rdi
	movq %rax, %rsi
	call insere
L373:
	movq $0, %rax
L372:
	movq %rbp, %rsp
	popq %rbp
	ret
L383:
	movq %rax, %rdi
	movq $0, %rsi
	movq $0, %rdx
	call make
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %r10
	movq %rax, 16(%r10)
	jmp L373
L403:
	movq -8(%rbp), %rcx
	movq 8(%rcx), %rdi
	movq $0, %rdx
	cmpq %rdx, %rdi
	sete %dil
	testq %rdi, %rdi
	jnz L398
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rdi
	movq %rax, %rsi
	call insere
	jmp L373
L398:
	movq $0, %rsi
	movq $0, %rdx
	movq %rax, %rdi
	call make
	movq -8(%rbp), %r10
	movq %r10, -24(%rbp)
	movq -24(%rbp), %r10
	movq %rax, 8(%r10)
	jmp L373
L409:
	movq $0, %rax
	jmp L372
print:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq %rdi, -8(%rbp)
	movq $40, %rdi
	call putchar
	movq -8(%rbp), %r9
	movq 8(%r9), %rcx
	movq $0, %rdi
	cmpq %rdi, %rcx
	setne %cl
	testq %rcx, %rcx
	jnz L350
L347:
	movq -8(%rbp), %rsi
	movq 0(%rsi), %rdi
	call print_int
	movq -8(%rbp), %rdx
	movq 16(%rdx), %rdi
	movq $0, %r9
	cmpq %r9, %rdi
	setne %dil
	testq %rdi, %rdi
	jnz L339
L336:
	movq $41, %rdi
	call putchar
	movq %rbp, %rsp
	popq %rbp
	ret
L339:
	movq -8(%rbp), %rdx
	movq 16(%rdx), %rdi
	call print
	jmp L336
L350:
	movq -8(%rbp), %rax
	movq 8(%rax), %rdi
	call print
	jmp L347
	.data

